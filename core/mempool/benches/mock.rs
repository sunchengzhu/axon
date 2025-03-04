use std::sync::Arc;

use dashmap::DashMap;

use common_crypto::{
    Crypto, PrivateKey, Secp256k1Recoverable, Secp256k1RecoverablePrivateKey,
    Secp256k1RecoverablePublicKey, Signature, ToPublicKey, UncompressedPublicKey,
};

use protocol::rand::{random, rngs::OsRng};
use protocol::traits::{Context, MemPool, MemPoolAdapter};
use protocol::types::{
    public_to_address, recover_intact_pub_key, Bytes, Eip1559Transaction, Hash, PackedTxHashes,
    Public, SignedTransaction, TransactionAction, UnsignedTransaction, UnverifiedTransaction, H160,
    H256, U256,
};
use protocol::{async_trait, tokio, ProtocolResult};

use core_executor::system_contract::system_contract_address;
use core_mempool::{AdapterError, MemPoolError, MemPoolImpl};

pub const CYCLE_LIMIT: u64 = 1_000_000;
pub const TX_NUM_LIMIT: u64 = 10_000;
pub const CURRENT_HEIGHT: u64 = 999;
pub const POOL_SIZE: usize = 100_000;
pub const MAX_TX_SIZE: u64 = 1024; // 1KB
pub const TIMEOUT: u64 = 1000;
pub const TIMEOUT_GAP: u64 = 100;
pub const NATIVE_TOKEN_ISSUE_ADDRESS: H160 = system_contract_address(0x0);

pub struct HashMemPoolAdapter {
    network_txs: DashMap<Hash, SignedTransaction>,
}

impl HashMemPoolAdapter {
    fn new() -> HashMemPoolAdapter {
        HashMemPoolAdapter {
            network_txs: DashMap::new(),
        }
    }
}

#[async_trait]
impl MemPoolAdapter for HashMemPoolAdapter {
    async fn pull_txs(
        &self,
        _ctx: Context,
        _height: Option<u64>,
        tx_hashes: Vec<Hash>,
    ) -> ProtocolResult<Vec<SignedTransaction>> {
        let mut vec = Vec::with_capacity(tx_hashes.len());
        for hash in tx_hashes {
            if let Some(tx) = self.network_txs.get(&hash) {
                vec.push(tx.clone());
            }
        }
        Ok(vec)
    }

    async fn broadcast_tx(
        &self,
        _ctx: Context,
        _origin: Option<usize>,
        tx: SignedTransaction,
    ) -> ProtocolResult<()> {
        self.network_txs.insert(tx.transaction.hash, tx);
        Ok(())
    }

    async fn check_authorization(
        &self,
        _ctx: Context,
        _tx: &SignedTransaction,
    ) -> ProtocolResult<U256> {
        Ok(U256::zero())
    }

    async fn check_transaction(&self, _ctx: Context, tx: &SignedTransaction) -> ProtocolResult<()> {
        check_hash(tx)?;
        check_sig(tx)
    }

    async fn check_storage_exist(&self, _ctx: Context, _tx_hash: &Hash) -> ProtocolResult<()> {
        Ok(())
    }

    async fn get_latest_height(&self, _ctx: Context) -> ProtocolResult<u64> {
        Ok(CURRENT_HEIGHT)
    }

    async fn get_transactions_from_storage(
        &self,
        _ctx: Context,
        _height: Option<u64>,
        _tx_hashes: &[Hash],
    ) -> ProtocolResult<Vec<Option<SignedTransaction>>> {
        Ok(vec![])
    }

    fn clear_nonce_cache(&self) {}

    fn set_args(&self, _context: Context, _state_root: H256, _gas_limit: u64, _max_tx_size: u64) {}

    fn report_good(&self, _ctx: Context) {}
}

pub fn default_mock_txs(size: usize) -> Vec<SignedTransaction> {
    mock_txs(size, 0, TIMEOUT)
}

pub fn mock_txs(valid_size: usize, invalid_size: usize, timeout: u64) -> Vec<SignedTransaction> {
    (0..valid_size + invalid_size)
        .map(|i| {
            let priv_key = Secp256k1RecoverablePrivateKey::generate(&mut OsRng);
            let pub_key = priv_key.pub_key();
            mock_signed_tx(&priv_key, &pub_key, timeout, i as u64, i < valid_size)
        })
        .collect()
}

pub async fn new_mempool(
    pool_size: usize,
    _timeout_gap: u64,
    _cycles_limit: u64,
    _max_tx_size: u64,
) -> MemPoolImpl<HashMemPoolAdapter> {
    let adapter = HashMemPoolAdapter::new();
    MemPoolImpl::new(pool_size, 20, adapter, vec![]).await
}

pub async fn default_mempool() -> MemPoolImpl<HashMemPoolAdapter> {
    new_mempool(POOL_SIZE, TIMEOUT_GAP, CYCLE_LIMIT, MAX_TX_SIZE).await
}

pub fn mock_transaction(nonce: u64, is_call_system_script: bool) -> Eip1559Transaction {
    Eip1559Transaction {
        nonce:                    nonce.into(),
        gas_limit:                U256::one(),
        max_priority_fee_per_gas: U256::one(),
        gas_price:                U256::one(),
        action:                   if is_call_system_script {
            TransactionAction::Call(NATIVE_TOKEN_ISSUE_ADDRESS)
        } else {
            TransactionAction::Create
        },
        value:                    U256::one(),
        data:                     random_bytes(32).to_vec().into(),
        access_list:              vec![],
    }
}

pub fn mock_signed_tx(
    priv_key: &Secp256k1RecoverablePrivateKey,
    pub_key: &Secp256k1RecoverablePublicKey,
    _timeout: u64,
    nonce: u64,
    valid: bool,
) -> SignedTransaction {
    let raw = mock_transaction(nonce, false);
    let mut tx = UnverifiedTransaction {
        unsigned:  UnsignedTransaction::Eip1559(raw),
        signature: None,
        chain_id:  Some(random::<u64>()),
        hash:      Default::default(),
    };

    let signature = if valid {
        Secp256k1Recoverable::sign_message(tx.signature_hash(true).as_bytes(), &priv_key.to_bytes())
            .unwrap()
            .to_bytes()
    } else {
        Bytes::copy_from_slice([0u8; 65].as_ref())
    };

    tx.signature = Some(signature.into());

    let pub_key = Public::from_slice(&pub_key.to_uncompressed_bytes()[1..65]);

    SignedTransaction {
        transaction: tx.calc_hash(),
        sender:      public_to_address(&pub_key),
        public:      Some(pub_key),
    }
}

fn random_bytes(len: usize) -> Bytes {
    Bytes::from((0..len).map(|_| random::<u8>()).collect::<Vec<_>>())
}

fn check_hash(tx: &SignedTransaction) -> ProtocolResult<()> {
    assert!(tx.transaction.signature.is_some());
    let tx_clone = tx.transaction.clone();
    let calc_hash = tx_clone.calc_hash().hash;

    if calc_hash != tx.transaction.hash {
        return Err(MemPoolError::CheckHash {
            expect: calc_hash,
            actual: tx.transaction.hash,
        }
        .into());
    }
    Ok(())
}

pub fn check_sig(stx: &SignedTransaction) -> ProtocolResult<()> {
    Secp256k1Recoverable::verify_signature(
        stx.transaction.signature_hash(true).as_bytes(),
        stx.transaction
            .signature
            .as_ref()
            .unwrap()
            .as_bytes()
            .as_ref(),
        recover_intact_pub_key(&stx.public.unwrap()).as_bytes(),
    )
    .map_err(|err| AdapterError::VerifySignature(err.to_string()))?;
    Ok(())
}

pub fn default_mempool_sync() -> MemPoolImpl<HashMemPoolAdapter> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(default_mempool())
}

pub async fn concurrent_insert(
    txs: Vec<SignedTransaction>,
    mempool: Arc<MemPoolImpl<HashMemPoolAdapter>>,
) {
    let futs = txs
        .into_iter()
        .map(|tx| {
            let mempool = Arc::clone(&mempool);
            tokio::spawn(async { exec_insert(tx, mempool).await })
        })
        .collect::<Vec<_>>();
    let _ = futures::future::try_join_all(futs).await;
}

pub async fn exec_get_full_txs(
    require_hashes: Vec<Hash>,
    mempool: Arc<MemPoolImpl<HashMemPoolAdapter>>,
) -> Vec<SignedTransaction> {
    mempool
        .get_full_txs(Context::new(), None, &require_hashes)
        .await
        .unwrap()
}

pub async fn exec_insert(
    signed_tx: SignedTransaction,
    mempool: Arc<MemPoolImpl<HashMemPoolAdapter>>,
) {
    if let Err(e) = mempool.insert(Context::new(), signed_tx).await {
        println!("{:?}", e);
    }
}

pub async fn concurrent_check_sig(txs: Vec<SignedTransaction>) {
    let futs = txs
        .into_iter()
        .map(|tx| tokio::task::spawn_blocking(move || check_sig(&tx)))
        .collect::<Vec<_>>();

    let _ = futures::future::try_join_all(futs).await;
}

pub async fn exec_flush(remove_hashes: Vec<Hash>, mempool: Arc<MemPoolImpl<HashMemPoolAdapter>>) {
    mempool
        .flush(Context::new(), &remove_hashes, 0)
        .await
        .unwrap()
}

pub async fn exec_package(
    mempool: Arc<MemPoolImpl<HashMemPoolAdapter>>,
    cycle_limit: U256,
    tx_num_limit: u64,
) -> PackedTxHashes {
    mempool
        .package(Context::new(), cycle_limit, tx_num_limit)
        .await
        .unwrap()
}

use ckb_types::{packed, prelude::*};

use protocol::types::MerkleRoot;

use crate::system_contract::image_cell::abi::image_cell_abi;
use crate::system_contract::image_cell::error::ImageCellResult;
use crate::system_contract::image_cell::store::{
    cell_key, commit, get_cell, header_key, insert_cell, insert_header, remove_cell,
    remove_header as remove_h, CellInfo,
};
use crate::system_contract::image_cell::trie_db::RocksTrieDB;
use crate::MPTTrie;

pub fn update(
    mpt: &mut MPTTrie<RocksTrieDB>,
    data: image_cell_abi::UpdateCall,
) -> ImageCellResult<MerkleRoot> {
    save_cells(mpt, data.outputs, data.header.number)?;

    mark_cells_consumed(mpt, data.inputs, data.header.number)?;

    save_header(mpt, &data.header)?;

    commit(mpt)
}

pub fn rollback(
    mpt: &mut MPTTrie<RocksTrieDB>,
    data: image_cell_abi::RollbackCall,
) -> ImageCellResult<MerkleRoot> {
    remove_cells(mpt, data.outputs)?;

    mark_cells_not_consumed(mpt, data.inputs)?;

    remove_header(mpt, data.block_number, &data.block_hash)?;

    commit(mpt)
}

fn save_cells(
    mpt: &mut MPTTrie<RocksTrieDB>,
    outputs: Vec<image_cell_abi::CellInfo>,
    created_number: u64,
) -> ImageCellResult<()> {
    for cell in outputs {
        let lock = cell.output.lock;
        let lock = packed::Script::new_builder()
            .args(lock.args.0.pack())
            .code_hash(lock.code_hash.pack())
            .hash_type(lock.hash_type.into())
            .build();

        let type_builder = packed::ScriptOpt::new_builder();
        let type_ = if !cell.output.type_.is_empty() {
            let type_ = &cell.output.type_[0];
            let type_ = packed::Script::new_builder()
                .args(type_.args.0.pack())
                .code_hash(type_.code_hash.pack())
                .hash_type(type_.hash_type.into())
                .build();
            type_builder.set(Some(type_))
        } else {
            type_builder
        }
        .build();

        let cell_output = packed::CellOutput::new_builder()
            .lock(lock)
            .type_(type_)
            .capacity(cell.output.capacity.pack())
            .build();

        let cell_info = CellInfo {
            cell_output: cell_output.as_bytes(),
            cell_data: cell.data.0,
            created_number,
            consumed_number: None,
        };

        let key = cell_key(&cell.out_point.tx_hash, cell.out_point.index);

        insert_cell(mpt, &key, &cell_info)?;
    }
    Ok(())
}

fn mark_cells_consumed(
    mpt: &mut MPTTrie<RocksTrieDB>,
    inputs: Vec<image_cell_abi::OutPoint>,
    consumed_number: u64,
) -> ImageCellResult<()> {
    for input in inputs {
        let key = cell_key(&input.tx_hash, input.index);

        if let Some(ref mut cell) = get_cell(mpt, &key)? {
            cell.consumed_number = Some(consumed_number);
            insert_cell(mpt, &key, cell)?;
        }
    }
    Ok(())
}

fn save_header(
    mpt: &mut MPTTrie<RocksTrieDB>,
    header: &image_cell_abi::Header,
) -> ImageCellResult<()> {
    let raw = packed::RawHeader::new_builder()
        .compact_target(header.compact_target.pack())
        .dao(header.dao.pack())
        .epoch(header.epoch.pack())
        .extra_hash(header.block_hash.pack())
        .number(header.number.pack())
        .parent_hash(header.parent_hash.pack())
        .proposals_hash(header.proposals_hash.pack())
        .timestamp(header.timestamp.pack())
        .transactions_root(header.transactions_root.pack())
        .version(header.version.pack())
        .build();

    let packed_header = packed::Header::new_builder()
        .raw(raw)
        .nonce(header.nonce.pack())
        .build();

    let key = header_key(&header.block_hash, header.number);

    insert_header(mpt, &key, &packed_header)
}

fn remove_cells(
    mpt: &mut MPTTrie<RocksTrieDB>,
    outputs: Vec<image_cell_abi::OutPoint>,
) -> ImageCellResult<()> {
    for output in outputs {
        let key = cell_key(&output.tx_hash, output.index);
        remove_cell(mpt, &key)?;
    }
    Ok(())
}

fn mark_cells_not_consumed(
    mpt: &mut MPTTrie<RocksTrieDB>,
    inputs: Vec<image_cell_abi::OutPoint>,
) -> ImageCellResult<()> {
    for input in inputs {
        let key = cell_key(&input.tx_hash, input.index);
        if let Some(ref mut cell) = get_cell(mpt, &key)? {
            cell.consumed_number = None;
            insert_cell(mpt, &key, cell)?;
        }
    }
    Ok(())
}

fn remove_header(
    mpt: &mut MPTTrie<RocksTrieDB>,
    block_number: u64,
    block_hash: &[u8; 32],
) -> ImageCellResult<()> {
    let key = header_key(block_hash, block_number);
    remove_h(mpt, &key)
}
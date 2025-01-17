global main:
    // First, initialise the shift table
    %shift_table_init

    // Second, load all MPT data from the prover.
    PUSH hash_initial_tries
    %jump(load_all_mpts)

global hash_initial_tries:
    %mpt_hash_state_trie   %mload_global_metadata(@GLOBAL_METADATA_STATE_TRIE_DIGEST_BEFORE)    %assert_eq
    %mpt_hash_txn_trie     %mload_global_metadata(@GLOBAL_METADATA_TXN_TRIE_DIGEST_BEFORE)      %assert_eq
    %mpt_hash_receipt_trie %mload_global_metadata(@GLOBAL_METADATA_RECEIPT_TRIE_DIGEST_BEFORE)  %assert_eq

global start_txns:
    // stack: (empty)
    // Last mpt input is txn_nb.
    PROVER_INPUT(mpt)
    PUSH 0
    // stack: init_used_gas, txn_nb

txn_loop:
    // If the prover has no more txns for us to process, halt.
    PROVER_INPUT(end_of_txns)
    %jumpi(hash_final_tries)

    // Call route_txn. When we return, continue the txn loop.
    PUSH txn_loop_after
    // stack: retdest, prev_used_gas, txn_nb
    %jump(route_txn)

global txn_loop_after:
    // stack: success, leftover_gas, cur_cum_gas, txn_nb
    %process_receipt
    // stack: new_cum_gas, txn_nb
    SWAP1 %increment SWAP1
    %jump(txn_loop)

global hash_final_tries:
    // stack: cum_gas, txn_nb
    %pop2
    %mpt_hash_state_trie   %mload_global_metadata(@GLOBAL_METADATA_STATE_TRIE_DIGEST_AFTER)     %assert_eq
    %mpt_hash_txn_trie     %mload_global_metadata(@GLOBAL_METADATA_TXN_TRIE_DIGEST_AFTER)       %assert_eq
    %mpt_hash_receipt_trie %mload_global_metadata(@GLOBAL_METADATA_RECEIPT_TRIE_DIGEST_AFTER)   %assert_eq
    %jump(halt)

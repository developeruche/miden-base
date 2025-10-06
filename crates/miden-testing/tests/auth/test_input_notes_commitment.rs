use miden_objects::crypto::merkle::SparseMerklePath;
use miden_objects::note::{Note, NoteInclusionProof};
use miden_objects::transaction::{InputNote, InputNotes};
use miden_objects::{Felt, Word};

/// Test that demonstrates the difference in INPUT_NOTES_COMMITMENT between
/// authenticated and unauthenticated notes.
///
/// This test directly shows that the same notes produce different commitments
/// depending on whether they have inclusion proofs (authenticated) or not.
#[tokio::test]
async fn test_input_notes_commitment_difference() -> anyhow::Result<()> {
    // Create a simple test note
    let note = Note::mock_noop(Word::from([Felt::new(1), Felt::new(2), Felt::new(3), Felt::new(4)]));
    
    println!("=== Testing INPUT_NOTES_COMMITMENT Difference ===\n");
    println!("Note ID: {:?}", note.id());
    println!("Note Nullifier: {:?}", note.nullifier());
    println!("Note Commitment: {:?}", note.commitment());
    
    // Test 1: Create InputNotes with UNAUTHENTICATED note
    println!("\n--- Test 1: Unauthenticated Note ---");
    let unauthenticated_note = InputNote::Unauthenticated { note: note.clone() };
    let unauthenticated_notes = InputNotes::new(vec![unauthenticated_note.clone()])?;
    let unauthenticated_commitment = unauthenticated_notes.commitment();
    println!("Unauthenticated InputNotes commitment: {:?}", unauthenticated_commitment);
    
    // Test 2: Create InputNotes with AUTHENTICATED note (with mock proof)
    println!("\n--- Test 2: Authenticated Note ---");
    // Create a mock inclusion proof
    let block_num = 1u32.into(); // BlockNumber implements From<u32>
    let node_index = 0u16;
    
    // Create a mock sparse merkle path with empty nodes mask and no nodes
    let mock_path = SparseMerklePath::from_parts(0, vec![])?;
    
    let mock_proof = NoteInclusionProof::new(
        block_num,
        node_index,
        mock_path,
    )?;
    
    let authenticated_note = InputNote::Authenticated { 
        note: note.clone(), 
        proof: mock_proof 
    };
    let authenticated_notes = InputNotes::new(vec![authenticated_note.clone()])?;
    let authenticated_commitment = authenticated_notes.commitment();
    println!("Authenticated InputNotes commitment: {:?}", authenticated_commitment);
    
    // Test 3: Using the From<Vec<Note>> implementation (creates unauthenticated)
    println!("\n--- Test 3: From Vec<Note> (Unauthenticated) ---");
    let notes_from_vec: InputNotes<InputNote> = vec![note.clone()].into();
    let from_vec_commitment = notes_from_vec.commitment();
    println!("From Vec<Note> commitment: {:?}", from_vec_commitment);
    
    // Verify the commitments are different
    println!("\n=== RESULTS ===");
    println!("Authenticated vs Unauthenticated commitments are DIFFERENT: {}", 
        authenticated_commitment != unauthenticated_commitment);
    println!("From Vec<Note> equals Unauthenticated: {}", 
        from_vec_commitment == unauthenticated_commitment);
    
    // The commitments MUST be different because:
    // - Authenticated notes contribute (nullifier, EMPTY_WORD) to the hash
    // - Unauthenticated notes contribute (nullifier, note_commitment) to the hash
    assert_ne!(
        authenticated_commitment, 
        unauthenticated_commitment,
        "Authenticated and unauthenticated notes should produce different INPUT_NOTES_COMMITMENT"
    );
    
    assert_eq!(
        from_vec_commitment,
        unauthenticated_commitment,
        "Vec<Note>::into() should produce unauthenticated notes"
    );
    
    println!("\n✓ Test confirmed: INPUT_NOTES_COMMITMENT differs based on authentication status");
    println!("This is the root cause of the 'Unauthorized' error in multisig transactions");
    
    Ok(())
}
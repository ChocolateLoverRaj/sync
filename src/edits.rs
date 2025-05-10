use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use tink_core::keyset::Handle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EditId {
    pub user_id: u64,
    pub counter: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Edit {
    pub id: EditId,
    pub change: i64,
}

// pub fn t() -> Result<(), Box<dyn Error>> {
//     tink_signature::init();
//     // Other key templates can also be used.
//     let kh = tink_core::keyset::Handle::new(&tink_signature::ecdsa_p256_key_template())?;
//     let s = tink_signature::new_signer(&kh)?;

//     let pt = b"this data needs to be signed";
//     let a = s.sign(pt)?;
//     println!("'{}' => {}", String::from_utf8_lossy(pt), hex::encode(&a));

//     let pubkh = kh.public()?;
//     let v = tink_signature::new_verifier(&pubkh)?;
//     v.verify(&a, b"this data needfs to be signed")?;
//     println!("Signature verified.");
//     Ok(())
// }

pub struct ExternalEdits {
    encrypted_serialized_edits: Vec<u8>,
    signature: Vec<u8>,
}

pub fn get_external_edits(encryption_key: &Handle, private_key: &Handle) -> ExternalEdits {
    let signer = tink_signature::new_signer(&private_key).unwrap();
    let edits = HashSet::<Edit>::from_iter([
        Edit {
            id: EditId {
                user_id: 1,
                counter: 0,
            },
            change: 1,
        },
        Edit {
            id: EditId {
                user_id: 1,
                counter: 1,
            },
            change: 1,
        },
    ]);
    let serialized_edits = postcard::to_allocvec(&edits).unwrap();
    let a = tink_aead::new(encryption_key).unwrap();
    let encrypted_edits = a.encrypt(&serialized_edits, Default::default()).unwrap();
    ExternalEdits {
        signature: signer.sign(&encrypted_edits).unwrap(),
        encrypted_serialized_edits: encrypted_edits,
    }
}

pub fn deserialize_edits(
    encryption_key: &Handle,
    public_key: &Handle,
    external_edits: &ExternalEdits,
) -> HashSet<Edit> {
    let verifier = tink_signature::new_verifier(&public_key).unwrap();
    verifier
        .verify(
            &external_edits.signature,
            &external_edits.encrypted_serialized_edits,
        )
        .unwrap();
    let a = tink_aead::new(encryption_key).unwrap();
    let serialized_edits = a
        .decrypt(
            &external_edits.encrypted_serialized_edits,
            Default::default(),
        )
        .unwrap();
    postcard::from_bytes(&serialized_edits).unwrap()
}

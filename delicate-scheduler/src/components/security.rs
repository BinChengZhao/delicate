use crate::prelude::*;

#[test]
fn test_rsa_crypt(){
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RSAPublicKey::from(&priv_key);
    
    // Encrypt
    let data = b"hello world";
    let enc_data = pub_key.encrypt(&mut rng, PaddingScheme::new_pkcs1v15_encrypt(), &data[..]).expect("failed to encrypt");
    assert_ne!(&data[..], &enc_data[..]);
    
    // Decrypt
    let dec_data = priv_key.decrypt(PaddingScheme::new_pkcs1v15_encrypt(), &enc_data).expect("failed to decrypt");
    assert_eq!(&data[..], &dec_data[..]);
}


#[test]
fn test_rsa_sign(){
    let mut rng = OsRng;
    let bits = 2048;
    let priv_key = RSAPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RSAPublicKey::from(&priv_key);
    
    // Verify
    let data = b"hello world";

    // Sign
    let sign_data = priv_key.sign(PaddingScheme::new_pkcs1v15_sign(None), data).expect("failed to sign");
    assert_ne!(&data[..], &sign_data[..]);

    pub_key.verify( PaddingScheme::new_pkcs1v15_sign(None), data, &sign_data[..]).expect("failed to Verify");
}
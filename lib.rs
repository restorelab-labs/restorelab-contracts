#![no_std]
use soroban_sdk::{contract,contractimpl,contracttype,Address,BytesN,Env};
#[contracttype]
#[derive(Clone)]
pub enum Key {Owner,Record,Hint}
#[contract]
pub struct Specimen;
#[contractimpl]
impl Specimen {
 pub fn __constructor(env:Env,owner:Address){env.storage().instance().set(&Key::Owner,&owner);}
 pub fn put(env:Env,digest:BytesN<32>){let owner:Address=env.storage().instance().get(&Key::Owner).unwrap();owner.require_auth();env.storage().persistent().set(&Key::Record,&digest);}
 pub fn record(env:Env)->Option<BytesN<32>>{env.storage().persistent().get(&Key::Record)}
 pub fn hint(env:Env,digest:BytesN<32>){let owner:Address=env.storage().instance().get(&Key::Owner).unwrap();owner.require_auth();env.storage().temporary().set(&Key::Hint,&digest);}
 pub fn read_hint(env:Env)->Option<BytesN<32>>{env.storage().temporary().get(&Key::Hint)}
 pub fn maintain(env:Env,extend_to:u32){assert!(extend_to>0 && extend_to<=env.storage().max_ttl(),"invalid extension");env.storage().instance().extend_ttl(extend_to,extend_to);if env.storage().persistent().has(&Key::Record){env.storage().persistent().extend_ttl(&Key::Record,extend_to,extend_to);}}
}
#[cfg(test)]
mod tests {
use super::*;use soroban_sdk::testutils::{Address as _,Ledger,storage::{Persistent,Temporary,Instance}};
fn setup()->(Env,Address){let e=Env::default();e.ledger().with_mut(|li|{li.sequence_number=100;li.min_persistent_entry_ttl=500;li.min_temp_entry_ttl=10;li.max_entry_ttl=20000;});e.mock_all_auths();let owner=Address::generate(&e);let id=e.register(Specimen,(&owner,));(e,id)}
#[test]fn durable_digest_round_trip(){let(e,id)=setup();let c=SpecimenClient::new(&e,&id);let d=BytesN::from_array(&e,&[7;32]);c.put(&d);assert_eq!(c.record(),Some(d));}
#[test]fn temporary_hint_expires(){let(e,id)=setup();let c=SpecimenClient::new(&e,&id);c.hint(&BytesN::from_array(&e,&[1;32]));e.ledger().with_mut(|li|li.sequence_number+=11);assert_eq!(c.read_hint(),None);}
#[test]fn maintenance_extends_instance_and_durable_entry(){let(e,id)=setup();let c=SpecimenClient::new(&e,&id);c.put(&BytesN::from_array(&e,&[1;32]));c.maintain(&1000);e.as_contract(&id,||{assert_eq!(e.storage().persistent().get_ttl(&Key::Record),1000);assert_eq!(e.storage().instance().get_ttl(),1000)});}
#[test]fn extension_does_not_keep_temporary_hints(){let(e,id)=setup();let c=SpecimenClient::new(&e,&id);c.hint(&BytesN::from_array(&e,&[1;32]));c.maintain(&1000);e.as_contract(&id,||assert_eq!(e.storage().temporary().get_ttl(&Key::Hint),9));}
#[test]#[should_panic]fn overwrite_requires_owner_auth(){let(e,id)=setup();e.mock_auths(&[]);SpecimenClient::new(&e,&id).put(&BytesN::from_array(&e,&[2;32]));}
#[test]#[should_panic]fn rejects_zero_extension(){let(e,id)=setup();SpecimenClient::new(&e,&id).maintain(&0);}
#[test]fn persistent_access_after_expiry_retains_digest_in_host(){let(e,id)=setup();let c=SpecimenClient::new(&e,&id);let d=BytesN::from_array(&e,&[9;32]);c.put(&d);c.maintain(&10000);e.as_contract(&id,||e.storage().persistent().extend_ttl(&Key::Record,10000,10000));e.ledger().with_mut(|li|li.sequence_number+=10001);assert_eq!(c.record(),Some(d));}
}

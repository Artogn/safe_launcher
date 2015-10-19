// Copyright 2015 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under (1) the MaidSafe.net Commercial License,
// version 1.0 or later, or (2) The General Public License (GPL), version 3, depending on which
// licence you accepted on initial access to the Software (the "Licences").
//
// By contributing code to the SAFE Network Software, or to this project generally, you agree to be
// bound by the terms of the MaidSafe Contributor Agreement, version 1.0.  This, along with the
// Licenses can be found in the root directory of this project at LICENSE, COPYING and CONTRIBUTOR.
//
// Unless required by applicable law or agreed to in writing, the SAFE Network Software distributed
// under the GPL Licence is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.
//
// Please review the Licences for the specific language governing permissions and limitations
// relating to use of the SAFE Network Software.

pub type ResponseType = Result<Option<::rustc_serialize::json::Json>, ::errors::LauncherError>;

mod dns;
mod nfs;
mod traits;

pub fn begin_parse<D>(client : ::std::sync::Arc<::std::sync::Mutex<::safe_core::client::Client>>,
                      decoder: &mut D) -> ResponseType
                                          where D: ::rustc_serialize::Decoder {
    unimplemented!()
}

fn version_parser<D:>(client             : ::std::sync::Arc<::std::sync::Mutex<::safe_core::client::Client>>,
                      mut endpoint_tokens: Vec<String>,
                      decoder            : &mut D) -> ResponseType
                                                      where D: ::rustc_serialize::Decoder {
    unimplemented!()
}

fn module_dispatcher<D>(client              : ::std::sync::Arc<::std::sync::Mutex<::safe_core::client::Client>>,
                        mut remaining_tokens: Vec<String>,
                        version             : f32,
                        decoder             : &mut D) -> ResponseType
                                                         where D: ::rustc_serialize::Decoder {
    unimplemented!()
}

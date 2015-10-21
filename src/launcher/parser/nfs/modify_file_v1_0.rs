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

#[derive(RustcDecodable, Debug)]
pub struct ModifyFile {
    is_path_shared : bool,
    path           : String,
    new_values     : OptionalParams,
}

impl ::launcher::parser::traits::Action for ModifyFile {
    fn execute(&mut self, params: ::launcher::parser::ParameterPacket) -> ::launcher::parser::ResponseType {
        use rustc_serialize::base64::FromBase64;

        if self.is_path_shared && !*eval_result!(params.safe_drive_access.lock()) {
            return Err(::errors::LauncherError::PermissionDenied)
        }

        let start_dir_key = if self.is_path_shared {
            &params.safe_drive_dir_key
        } else {
            &params.app_root_dir_key
        };

        let mut tokens = ::launcher::parser::helper::tokenise_path(&self.path, false);
        let file_name = try!(tokens.pop().ok_or(::errors::LauncherError::InvalidPath));
        let mut dir_of_file = try!(::launcher::parser::helper::get_final_subdirectory(params.client.clone(),
                                                                                      &tokens,
                                                                                      Some(start_dir_key)));

        let mut file= try!(dir_of_file.find_file(&file_name).map(|file| file.clone()).ok_or(::errors::LauncherError::InvalidPath));

        if self.new_values.name.is_none() &&
           self.new_values.user_metadata.is_none() &&
           self.new_values.content.is_none() {
            return Err(::errors::LauncherError::SpecificParseError("new_values could not be parsed or new_values is empty".to_string()));
        }

        let file_helper = ::safe_nfs::helper::file_helper::FileHelper::new(params.client);

        let mut metadata_updated = false;
        if let Some(ref name) = self.new_values.name {
            file.get_mut_metadata().set_name(name.clone());
            metadata_updated = true;
        }

        if let Some(ref metadata_base64) = self.new_values.user_metadata {
            let metadata = try!(parse_result!(metadata_base64.from_base64(), "Failed to convert from base64"));
            file.get_mut_metadata().set_user_metadata(metadata);
            metadata_updated = true;
        }

        if metadata_updated {

            let _ = try!(file_helper.update_metadata(file.clone(), &mut dir_of_file));
        }

        if let Some(ref file_content_params) = self.new_values.content {
            let mut mode = ::safe_nfs::helper::writer::Mode::Overwrite;
            if let Some(modify) = file_content_params.modify {
                if modify {
                    mode = ::safe_nfs::helper::writer::Mode::Modify;
                }
            };
            let offset = match mode {
                ::safe_nfs::helper::writer::Mode::Overwrite => 0,
                ::safe_nfs::helper::writer::Mode::Modify    => file_content_params.offset.map_or(0, |v| v)
            };
            if let Some(ref data) = file_content_params.bytes {
                let mut writer = try!(file_helper.update_content(file.clone(), mode, dir_of_file));
                let bytes = try!(parse_result!(data.from_base64(), "Failed to convert from base64"));
                writer.write(&bytes[..], offset);
                let _ = try!(writer.close());
            } else {
                return Err(::errors::LauncherError::Unexpected("Empty bytes received for updating file".to_string()));
            }
        }

        Ok(None)
    }
}

#[derive(Debug)]
struct OptionalParams {
    pub name                  : Option<String>,
    pub content               : Option<FileContentParams>,
    pub user_metadata         : Option<String>,
}

impl ::rustc_serialize::Decodable for OptionalParams {
    fn decode<D>(decoder: &mut D) -> Result<Self, D::Error>
                                     where D: ::rustc_serialize::Decoder {
        Ok(OptionalParams {
            name         : decoder.read_struct_field("dir_path", 0, |d| ::rustc_serialize::Decodable::decode(d)).ok(),
            content      : decoder.read_struct_field("content", 0, |d| ::rustc_serialize::Decodable::decode(d)).ok(),
            user_metadata: decoder.read_struct_field("user_metadata", 0, |d| ::rustc_serialize::Decodable::decode(d)).ok(),
        })
    }
}

#[derive(Debug)]
struct FileContentParams {
    pub offset: Option<u64>,
    pub modify: Option<bool>,
    pub bytes : Option<String>,
}

impl ::rustc_serialize::Decodable for FileContentParams {
    fn decode<D>(decoder: &mut D) -> Result<Self, D::Error>
                                     where D: ::rustc_serialize::Decoder {
        Ok(FileContentParams {
            offset: decoder.read_struct_field("offset", 0, |d| ::rustc_serialize::Decodable::decode(d)).ok(),
            modify: decoder.read_struct_field("offset", 0, |d| ::rustc_serialize::Decodable::decode(d)).ok(),
            bytes : decoder.read_struct_field("bytes", 0, |d| ::rustc_serialize::Decodable::decode(d)).ok(),
        })
    }
}

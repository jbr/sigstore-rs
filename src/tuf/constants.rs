//
// Copyright 2021 The Sigstore Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub(crate) static ref SIGSTORE_FULCIO_CERT_TARGET_REGEX: Regex =
        Regex::new(r#"fulcio(_v\d+)?\.crt\.pem"#).unwrap();
}

pub(crate) const SIGSTORE_METADATA_BASE: &str = "http://sigstore-tuf-root.storage.googleapis.com/";
pub(crate) const SIGSTORE_TARGET_BASE: &str =
    "http://sigstore-tuf-root.storage.googleapis.com/targets";

pub(crate) const SIGSTORE_REKOR_PUB_KEY_TARGET: &str = "rekor.pub";

pub(crate) const SIGSTORE_ROOT: &str = r#"{
	"signatures": [
		{
			"keyid": "2f64fb5eac0cf94dd39bb45308b98920055e9a0d8e012a7220787834c60aef97",
			"sig": "3046022100d3ea59490b253beae0926c6fa63f54336dea1ed700555be9f27ff55cd347639c0221009157d1ba012cead81948a4ab777d355451d57f5c4a2d333fc68d2e3f358093c2"
		},
		{
			"keyid": "bdde902f5ec668179ff5ca0dabf7657109287d690bf97e230c21d65f99155c62",
			"sig": "304502206eaef40564403ce572c6d062e0c9b0aab5e0223576133e081e1b495e8deb9efd02210080fd6f3464d759601b4afec596bbd5952f3a224cd06ed1cdfc3c399118752ba2"
		},
		{
			"keyid": "eaf22372f417dd618a46f6c627dbc276e9fd30a004fc94f9be946e73f8bd090b",
			"sig": "304502207baace02f56d8e6069f10b6ff098a26e7f53a7f9324ad62cffa0557bdeb9036c022100fb3032baaa090d0040c3f2fd872571c84479309b773208601d65948df87a9720"
		},
		{
			"keyid": "f40f32044071a9365505da3d1e3be6561f6f22d0e60cf51df783999f6c3429cb",
			"sig": "304402205180c01905505dd88acd7a2dad979dd75c979b3722513a7bdedac88c6ae8dbeb022056d1ddf7a192f0b1c2c90ff487de2fb3ec9f0c03f66ea937c78d3b6a493504ca"
		},
		{
			"keyid": "f505595165a177a41750a8e864ed1719b1edfccd5a426fd2c0ffda33ce7ff209",
			"sig": "3046022100c8806d4647c514d80fd8f707d3369444c4fd1d0812a2d25f828e564c99790e3f022100bb51f12e862ef17a7d3da2ac103bebc5c7e792237006c4cafacd76267b249c2f"
		}
	],
	"signed": {
		"_type": "root",
		"consistent_snapshot": false,
		"expires": "2022-05-11T19:09:02.663975009Z",
		"keys": {
			"2f64fb5eac0cf94dd39bb45308b98920055e9a0d8e012a7220787834c60aef97": {
				"keyid_hash_algorithms": [
					"sha256",
					"sha512"
				],
				"keytype": "ecdsa-sha2-nistp256",
				"keyval": {
					"public": "04cbc5cab2684160323c25cd06c3307178a6b1d1c9b949328453ae473c5ba7527e35b13f298b41633382241f3fd8526c262d43b45adee5c618fa0642c82b8a9803"
				},
				"scheme": "ecdsa-sha2-nistp256"
			},
			"b6710623a30c010738e64c5209d367df1c0a18cf90e6ab5292fb01680f83453d": {
				"keyid_hash_algorithms": [
					"sha256",
					"sha512"
				],
				"keytype": "ecdsa-sha2-nistp256",
				"keyval": {
					"public": "04fa1a3e42f2300cd3c5487a61509348feb1e936920fef2f83b7cd5dbe7ba045f538725ab8f18a666e6233edb7e0db8766c8dc336633449c5e1bbe0c182b02df0b"
				},
				"scheme": "ecdsa-sha2-nistp256"
			},
			"bdde902f5ec668179ff5ca0dabf7657109287d690bf97e230c21d65f99155c62": {
				"keyid_hash_algorithms": [
					"sha256",
					"sha512"
				],
				"keytype": "ecdsa-sha2-nistp256",
				"keyval": {
					"public": "04a71aacd835dc170ba6db3fa33a1a33dee751d4f8b0217b805b9bd3242921ee93672fdcfd840576c5bb0dc0ed815edf394c1ee48c2b5e02485e59bfc512f3adc7"
				},
				"scheme": "ecdsa-sha2-nistp256"
			},
			"eaf22372f417dd618a46f6c627dbc276e9fd30a004fc94f9be946e73f8bd090b": {
				"keyid_hash_algorithms": [
					"sha256",
					"sha512"
				],
				"keytype": "ecdsa-sha2-nistp256",
				"keyval": {
					"public": "04117b33dd265715bf23315e368faa499728db8d1f0a377070a1c7b1aba2cc21be6ab1628e42f2cdd7a35479f2dce07b303a8ba646c55569a8d2a504ba7e86e447"
				},
				"scheme": "ecdsa-sha2-nistp256"
			},
			"f40f32044071a9365505da3d1e3be6561f6f22d0e60cf51df783999f6c3429cb": {
				"keyid_hash_algorithms": [
					"sha256",
					"sha512"
				],
				"keytype": "ecdsa-sha2-nistp256",
				"keyval": {
					"public": "04cc1cd53a61c23e88cc54b488dfae168a257c34fac3e88811c55962b24cffbfecb724447999c54670e365883716302e49da57c79a33cd3e16f81fbc66f0bcdf48"
				},
				"scheme": "ecdsa-sha2-nistp256"
			},
			"f505595165a177a41750a8e864ed1719b1edfccd5a426fd2c0ffda33ce7ff209": {
				"keyid_hash_algorithms": [
					"sha256",
					"sha512"
				],
				"keytype": "ecdsa-sha2-nistp256",
				"keyval": {
					"public": "048a78a44ac01099890d787e5e62afc29c8ccb69a70ec6549a6b04033b0a8acbfb42ab1ab9c713d225cdb52b858886cf46c8e90a7f3b9e6371882f370c259e1c5b"
				},
				"scheme": "ecdsa-sha2-nistp256"
			},
			"fc61191ba8a516fe386c7d6c97d918e1d241e1589729add09b122725b8c32451": {
				"keyid_hash_algorithms": [
					"sha256",
					"sha512"
				],
				"keytype": "ecdsa-sha2-nistp256",
				"keyval": {
					"public": "044c7793ab74b9ddd713054e587b8d9c75c5f6025633d0fef7ca855ed5b8d5a474b23598fe33eb4a63630d526f74d4bdaec8adcb51993ed65652d651d7c49203eb"
				},
				"scheme": "ecdsa-sha2-nistp256"
			}
		},
		"roles": {
			"root": {
				"keyids": [
					"2f64fb5eac0cf94dd39bb45308b98920055e9a0d8e012a7220787834c60aef97",
					"bdde902f5ec668179ff5ca0dabf7657109287d690bf97e230c21d65f99155c62",
					"eaf22372f417dd618a46f6c627dbc276e9fd30a004fc94f9be946e73f8bd090b",
					"f40f32044071a9365505da3d1e3be6561f6f22d0e60cf51df783999f6c3429cb",
					"f505595165a177a41750a8e864ed1719b1edfccd5a426fd2c0ffda33ce7ff209"
				],
				"threshold": 3
			},
			"snapshot": {
				"keyids": [
					"fc61191ba8a516fe386c7d6c97d918e1d241e1589729add09b122725b8c32451"
				],
				"threshold": 1
			},
			"targets": {
				"keyids": [
					"2f64fb5eac0cf94dd39bb45308b98920055e9a0d8e012a7220787834c60aef97",
					"bdde902f5ec668179ff5ca0dabf7657109287d690bf97e230c21d65f99155c62",
					"eaf22372f417dd618a46f6c627dbc276e9fd30a004fc94f9be946e73f8bd090b",
					"f40f32044071a9365505da3d1e3be6561f6f22d0e60cf51df783999f6c3429cb",
					"f505595165a177a41750a8e864ed1719b1edfccd5a426fd2c0ffda33ce7ff209"
				],
				"threshold": 3
			},
			"timestamp": {
				"keyids": [
					"b6710623a30c010738e64c5209d367df1c0a18cf90e6ab5292fb01680f83453d"
				],
				"threshold": 1
			}
		},
		"spec_version": "1.0",
		"version": 2
	}
}"#;

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case("fulcio.crt.pem", true)]
    #[case("fulcio_v1.crt.pem", true)]
    #[case("fulcio-v2.crt.pem", false)]
    #[case("foo.crt.pem", false)]
    fn check_fulcio_regex(#[case] input: &str, #[case] matches: bool) {
        assert_eq!(SIGSTORE_FULCIO_CERT_TARGET_REGEX.is_match(input), matches);
    }
}

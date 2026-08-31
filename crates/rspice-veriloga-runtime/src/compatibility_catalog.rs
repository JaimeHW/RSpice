/// One explicitly authenticated migration from a format-26 generated model to
/// the current split-provenance contract.
///
/// Every target field and wire alias is exact. `semantic_identity_override_artifact`
/// is normally absent: generic semantic identities are derived directly from
/// the canonical source. Its one retained value permits an intentionally
/// stable historical semantic identity only for the exact generated artifact
/// named by that digest; matching source text alone is insufficient.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedVerilogACompatibilityCatalogEntry {
    pub module_name: &'static str,
    pub public_model_name: &'static str,
    pub source_identity: &'static str,
    pub target_descriptor_abi_version: u32,
    pub semantic_identity: &'static str,
    pub accepted_state_shape_identity: &'static str,
    pub semantic_identity_override_artifact: Option<&'static str>,
    pub wire_v26_combined_identity_alias: Option<&'static str>,
    pub wire_ui_v1_descriptor_signature_alias: Option<&'static str>,
}

const fn entry(
    module_name: &'static str,
    public_model_name: &'static str,
    source_identity: &'static str,
    semantic_identity: &'static str,
    accepted_state_shape_identity: &'static str,
    semantic_identity_override_artifact: Option<&'static str>,
    wire_v26_combined_identity_alias: Option<&'static str>,
    wire_ui_v1_descriptor_signature_alias: Option<&'static str>,
) -> GeneratedVerilogACompatibilityCatalogEntry {
    GeneratedVerilogACompatibilityCatalogEntry {
        module_name,
        public_model_name,
        source_identity,
        target_descriptor_abi_version: 3,
        semantic_identity,
        accepted_state_shape_identity,
        semantic_identity_override_artifact,
        wire_v26_combined_identity_alias,
        wire_ui_v1_descriptor_signature_alias,
    }
}

pub const GENERATED_VERILOGA_COMPATIBILITY_CATALOG:
    &[GeneratedVerilogACompatibilityCatalogEntry] = &[
    entry(
        "angelov",
        "angelov",
        "341f0b52a96b43fcfed883f716546e584697009fbc3e3a7faeac4b97c6402ef3",
        "b7aac2c5c9fdc481618a39d6e536a4b8bcb2710b3850e0d01905e7bca327f598",
        "4f733bdac2db948acc89a2e243c888b3a6568c4ebafeb3c3a876466fed1d1361",
        None,
        Some("55067ed69547d0bcba4a61ceaaa2f96f9bdca9c6b9b52901d556ed735ad2af21"),
        None,
    ),
    entry(
        "angelov_gan",
        "angelov_gan",
        "b339eb9f7c2a285588139c48678c9aa910f73673b6be140bb006f736f6f9b224",
        "a348fe4f00f480b493c53faa8aef1e70ceb83b0c3382c5e1ebf36e3e4cda8410",
        "379a1b3ab88263ee72078e0ac7d6d2127c04e288e75a902cd228b0c267ffbbc6",
        None,
        Some("894964a28c7a4869f0f424245bdd1e963a0fef5b0823af861d6cea61ec825a38"),
        None,
    ),
    entry(
        "asmesd",
        "asmesd",
        "3f2257ad01d855f57811b64d28af2686d1821615056192a640b81c0ad5587988",
        "5cf68182a22d2d1ac4ef1d5d1a67fcbb189c06bc4a13e1995f022a59d3d6b356",
        "daf937e7fa1f75974dc3f84344458fecc25ddc045532f09da410c752f700b1f1",
        None,
        Some("11575ca669f1ae2fc4015a4972e621657608233d49f67c463b652b16bea844bd"),
        None,
    ),
    entry(
        "asmesd_dio",
        "asmesd_dio",
        "e70e70f99ffe68a1df66612db3636598d5a344def41b62a8919eb78cb7d7e209",
        "3fd48d9c6766123a216d74ba392eb3ef4f1dbcf4cededb489584b2e0fefd7604",
        "c8fdec8f09ff4792c1649544722413b7ec4805c3a4ea69888fbe9127fa8699b8",
        None,
        Some("3b8b5b8efbba256848894d0436cea9299a3cf893e0ba820875a9ebdcbc55050d"),
        None,
    ),
    entry(
        "asmhemt",
        "asmhemt",
        "24cfac517f254258f9935000d2739f7fce9c91c16cdb6d3724d5f342752b9366",
        "87c53cb014209f44eb24254807056911d8a92f87f94a3826df617390ae1a6eff",
        "b62bb8b9024e6f2abbd8cea53ec9c0ec78d7b59d6170b87e7c83d97ade78f234",
        None,
        Some("d881a0d883ac21f45f81ac739b8aeac3a5d26c3592f2adee62dc3661ffa982f0"),
        None,
    ),
    entry(
        "bjt505_va",
        "bjt505_va",
        "47beb9eec6e043638d2258f2387fe9d436b0bc782069613456738fb4546e3b7d",
        "898487a6ce9c4be3b3daec1872fbbb5dde31ab7e2610b4f4dbad06f949088ee7",
        "1aff88b7754c205621bef558af73a570bb86298d90f2e942812895e3b6de545c",
        None,
        Some("f02f7f59fbada331f400a81d5e36aa39e5c6cc5fad32ca56d4bfa22469484ac5"),
        None,
    ),
    entry(
        "bjt505t_va",
        "bjt505t_va",
        "475491172ee230720ec264f8686ed06910b485d8466ed2ea8b1c3f5f35f6bd60",
        "a068c9c8cd695460938466d71e8d79f85db3aafbd37d2174e631e5aad2ed9001",
        "6ade4654fce8c1943d85bec81e139f2e38fc093854d5647001da4f0498030cab",
        None,
        Some("d84fa9765b71b8877a59fb38de3488cefd67438ebaffa18b5f2f155a224240c1"),
        None,
    ),
    entry(
        "bjtd505_va",
        "bjtd505_va",
        "c47af260a6bdd95492a405eed984c7d0c1d67dc63fb261c9be887152aa7349ac",
        "e87c9a1dc014d5d7083ef79c6f7a487be1014200f539c705623276f817a864f3",
        "825f1b7fc938c443589306c28ec72033a229aa8ec1d4542ef108d33151701733",
        None,
        Some("1dadc1f71ba9ec94268f783fc25081989043ae85e3fd3e6727afb6d4a81c484a"),
        None,
    ),
    entry(
        "bjtd505t_va",
        "bjtd505t_va",
        "0a31033f15140d83805adbb59f241188ec2e1128300c89f81ca8f11dfbbadb42",
        "930e32f872e39b5de39b30a5a4ca22956b215f382cd568c3430809c14c7bb6d2",
        "ed4bf8bf6f675ee3f9f6403782e38d47744194b4901d69ed4af7cad72969d8f0",
        None,
        Some("8d3a0e5bf2a4b53d24add52b2839ee3c0cfcbebd10eb95409232339b76a31c2f"),
        None,
    ),
    entry(
        "bsimbulk",
        "bsimbulk",
        "4873f625fe31b2e0741dfb308efde31bd3254f42eca4d4127257792d9bda0f73",
        "b9bbe832f6d7ceba702d5360cfaf4589d918564c0e10a3c064ff81ea7cb00403",
        "629324271623f21df5558217a6564054aad32595c6093235c7ecbf5ca94bf325",
        None,
        Some("1b6411e97525d6e58f83ecb5eed66870a17d60c0cc020a5729c35df0317f2ca9"),
        None,
    ),
    entry(
        "bsimcmg_va",
        "bsimcmg_va",
        "d25ffebc3132b5a43965a65ead7dc906127b352b80e1c96a308d0dcd2418d2f5",
        "d259efe62194a05fcf113b59b080a99394b4d7867c48c0f9eb2895657040dc95",
        "3207a2366cb796d5b014a2f31610ba8f350d833a11db29ff2c61a85494156c2b",
        None,
        Some("99fce4dfa8f1c5ed863f3ebb7ee0524546238509ab716a43b07a25d757341925"),
        None,
    ),
    entry(
        "bsimimg",
        "bsimimg",
        "745df14022db1cd60e551f3a254ec5b9fa9094d25a607c5cc7f75377967173d9",
        "674295708f4f886fb24f88b30f09d19c0ce9b44ecbdeaaf6af9abb3a078c9802",
        "943782651bc477d276b1d08942a2ff2646c0bd22ab4c42c7cff01378bd4e72c2",
        None,
        Some("e250a11bc9aebbc43d0b3c9a522a13a1a5285b1ac36cca0d38aba04cf8d71ebb"),
        None,
    ),
    entry(
        "bsimsoi",
        "bsimsoi__18c250bc",
        "023f80f8efdb5003f3803c813d2d4173f5f516de391fc31b7725cb46983537aa",
        "6166ad110e708f630b3a6d854134e0d03dc0601e9f4eeb3bc75b5b35e6f7e917",
        "5ebe40a2d62aede4038bd77ae329b8ceb9b06a373901da5141d4737f948a9bdd",
        None,
        Some("c0213a19cdadacf3f36ef72df9f94ebf9824a4a8d52b61c39c6bdb34d82695ce"),
        None,
    ),
    entry(
        "bsimsoi",
        "bsimsoi__e2aff994",
        "41847558a4494eb079b109fb059a2b4b037d9388a894b5281837f743cb1a1dcb",
        "945f77741c36ba7ce86a7b96cb984ec036920a85988e02911dbd729d71e9582b",
        "262a1e9da4fa7387c9884f0884bbdf1af168a660dc5b7360c38918c3f79974d6",
        None,
        Some("cdd5710a629a0b6770937ac3c27892c0e981ab96787e7cea1e6e78bd0283eaa5"),
        None,
    ),
    entry(
        "bsimsoi_va",
        "bsimsoi_va",
        "21f1e2b440907aff3efba02d05d0ee833aa857608d5e3b470b77b7e2a679037c",
        "57d649c8724c0c337ec68bff3c6c1d48c403ae61ed78ef880ca7eae33ff9cf17",
        "03f1114d09748fe48cfe48b09349acd9a592c0c0fda25fda5fa33e523c17d7a2",
        None,
        Some("feceed5085fdf06c9964360bce2e75b96b1294a9d6ca01fd17a6b01759774690"),
        None,
    ),
    entry(
        "DIODE_CMC",
        "DIODE_CMC",
        "bce3148c28a2756ac72af9006bcc7a70bfb4ab98a6c244cdc21b2b5279a243e0",
        "44e35a3a00550f8f364f08aa887d68b4158f5f3f6fe35d2fc0e8c6de11c24f0b",
        "99b1fa5ccfde3caf39390c9d20c9ad80ee1c840610de8ed15bddacd31381b5af",
        None,
        Some("64a8aac48a69ddf8623a4cfc4f452bf092a696f3c0352fd24d8b1ea0a5af9781"),
        None,
    ),
    entry(
        "ekv_va",
        "ekv_va",
        "f08696bcb49622e5ba502fa4bc3a6ade8eec5abd675323dafd18136393a8619f",
        "dca2e8151cfdc6a9d0a15def7a5db8770b50ed1b6a5549d675efa5a994bb4704",
        "73fea75dd7f48df86c3621e457f2f8d5e2d37ce002b503bb2fef7d6b449110ce",
        None,
        Some("4101f5adbf15e00d8478f3bfdfd74d40d1fbba7d8c0eb7abb319c798c2c28a4f"),
        None,
    ),
    entry(
        "ekv3_rf",
        "ekv3_rf",
        "4d3d934d75188a288254cac66695e69d23e7ca202ed1a7e76c47df5af288968e",
        "ac67027575ade6a73c96ad02e5f7622d72c33a16c8365fa2377e51b0ecd322ed",
        "debe56913b54147f246c0ef506c76642f8b3f81fffb3373adf827a693850594b",
        None,
        Some("a0e36624aa1f6b4e8bc64b8d6a69401165727740dac9f86cc684860329573cc3"),
        None,
    ),
    entry(
        "EPFL_HEMT_10a",
        "EPFL_HEMT_10a",
        "f5469f98a7a0b0678551905744a4443c697f0a1c131246aaa599173a5b06b772",
        "bdddf636918a20b013412b251d0333d6c24b1a1f2545f59929fc95d629789ef2",
        "1be160283a099b312fd9d83a68e007d40d7738ab5f46286e8adeb1af2613f05e",
        None,
        Some("3dd6c24b8cd6f6089b267a93662343a88badb8edaa2efb98e7d00f8693b24adc"),
        None,
    ),
    entry(
        "hicumL0va",
        "hicumL0va",
        "fd4960ff7f28f95f7ea3c3bb2442a8fcfccf5fdffc29c1e3e671bf5be55f2bcc",
        "540611a4dcd79a08a359411a60f2366cd1ee0f82c6cba536a3fe5aa38d491b5d",
        "ff68202cb561f8a5428590c5a2303f28258f0b48028a89f9c8ec6b1050abbc05",
        None,
        Some("ece0dcfd5883ed33a0c23fd8794447921be3fee9a321d37826f3520210e6d6fd"),
        None,
    ),
    entry(
        "hicumL2va",
        "hicumL2va",
        "590f61f3b5b38d9bc65af2f7b744854a19968ccb4a73b9ef49db7f043f59d575",
        "d0604d944d58d55fcd1d046d9f7209ee6e4b9936507a7d3af408390a4874c0bd",
        "ff24de97c68f59639a0099b1874413b3374a2c68dc460c7b4cac2b123ef4f594",
        None,
        Some("627ed43db5711469d0a26a896384c13d20ae3dab31d7e450586f6678b02eacb9"),
        None,
    ),
    entry(
        "hisimhv_n4_va",
        "hisimhv_n4_va",
        "b3e48d3b9538e5943d067c5843eefd177f25540fd02ee3d4c04b5d0f65950d68",
        "650550fd8b1f693e468dd208ba8ef0c2699deef2c75ea953a406ae5057e41b17",
        "2257cf253204eedafb3ba313f2361d27b247b643b1d50ee6701ddada23eb2f33",
        None,
        Some("c1501b2a2361246b99bee823656e37820d3046d28114d523486fd17f3c6c22de"),
        None,
    ),
    entry(
        "hisimhv_n5_va",
        "hisimhv_n5_va",
        "51684329505b48d61370258f1d00a3235c0cdf523f82ea8c0f4a92bb9953c931",
        "8300082a947b05a980c3af96e8caa4d958d9d6805cbff6fc226278cfc6a62290",
        "74525995d06293382d4723d18d4d0ff226d45eca7dd8352ab3ff55e9489f2750",
        None,
        Some("f22465f0b75ff4010e958f2dc56a6e42a5dd22a5b2e4abff821234e3afda2b37"),
        None,
    ),
    entry(
        "hisimhv_va",
        "hisimhv_va",
        "f7034d163951f352c07a95d17ec0841cdb585ee30eea5fd67a6631e96e9ecfd5",
        "6e23a29b580fac9e86424f37adf5dd5346bbd108d90a395c10c8d7a3b470628e",
        "2276da3d3c35e833dbd2ad09585d9ed646b38f94038c191af310ada86f5e9028",
        None,
        Some("c9be0c9c17ad6b46362d2b0631b3db00d3ac1e09bb33d4a8f450ec0086f54776"),
        None,
    ),
    entry(
        "hisimsoi_va",
        "hisimsoi_va__242bc21d",
        "62d2327172cae553530b1d48c88674ff9f538c6aa9665b24547daf6391343d83",
        "f805902c80a8c88204acb8cbb0c480bc8e88b2c2356b8c44884c3df1c4922cd1",
        "1bd35501d966eca39684e1c63660f075cbefd0f1bf75b281e12b1642942c917c",
        None,
        Some("6cb11056f63dbfd4400cacf7b9beb09ae3e303347052ea4a858c42cf1500a41d"),
        None,
    ),
    entry(
        "hisimsoi_va",
        "hisimsoi_va__38074d06",
        "07ce61cfd94d54fca74f8a5b3e585901180eca155ca50fa294722403403f0973",
        "4d7ca7e6027701e577bedfb2d6cde566362ae9237e2a0422406af3f9d3faf85e",
        "35cbe40df136ebc2aea2322aadf8e200a3aa5d9af18cea6c1661554dfc8dde5f",
        None,
        Some("9ed753a8c3f7995b19dd8221fb7b50dce2e7231cc3e937f66c2c0e3095daa7ed"),
        None,
    ),
    entry(
        "hisimsoi_va",
        "hisimsoi_va__5be18005",
        "9483fb9a342e2cf430882c66b2ffda26abe8d677e45ccd208d2f53107a2bfa60",
        "bfda4b1691d5209403857565d5c94260353303b766a6c0c8397d831ae15ffa24",
        "dfba4bd9bf276d6b54b870036b37b9a65caf1b2ac524444deb25f3c6d197414c",
        None,
        Some("0478736e86b7e9346efc97838aa869b44356e1d7c314bc70ec23158b58a5815d"),
        None,
    ),
    entry(
        "hisimsotb_va",
        "hisimsotb_va",
        "01cd10272aea8b2005eb21b2d8aa1e2281322196d778a51c55046440ff825a21",
        "708c234e799e85bb40d6c970c887a1570366249caa36ded96a1c2e2abbffde34",
        "7e1194de5e7d2ccf030cfcbd06342f90494411babf042ab655f3aa0e01a58e1f",
        None,
        Some("8746d710cce8864f0b8eaf59ede68a8fb4854099ec35296b51305e694cfe17b2"),
        None,
    ),
    entry(
        "JUNCAP200",
        "JUNCAP200",
        "0cb4ea0c1e72102d7effc3cb377e6092e889d591a97e7c88dde8d27a2294ec15",
        "f175581a51521e6343115f7fc3c6a0b76a7afd24eeb4320903ed102e52d189e9",
        "9407073b197ce9d39bc230c69ae9bad2702f9352073498fd24e06ded63f92587",
        None,
        Some("3c2764b9a28dda93b5ad5aafb601d0415f2f1cd3811f07c38aa0ec4f59a7b649"),
        None,
    ),
    entry(
        "l_utsoi",
        "l_utsoi__485e0ac9",
        "9704dbaabb653d777fb999b37e29e3d95fb0706cd7f92bdfa2c672d8b5a130d0",
        "52a6c1fae4607aaf9f7d18efaa58337403f67a2fbb7d00e2ab558b8b65250d96",
        "bf5628d461c03020e4c28b62c47a94920d7fe14164dd7d2512304a6a72582b85",
        None,
        Some("67a2595e0fd111c4ae1f3e960d833c55966a9caf256c2db72a37d1e977fdf11d"),
        None,
    ),
    entry(
        "l_utsoi",
        "l_utsoi__832ce87d",
        "bbb4f56a5a1a9b08a505d91c9eb3b5844bffa7205d3b2eef53a934f9dcc89dde",
        "79da5e601b3c861c982335fc4d74e11c2acc454e317e259929db61ca9d24224f",
        "0c329c93fdf08c4beae14dca34a3a9dacbe91cd74728b5efda3f122d28bbe07d",
        None,
        Some("72206167544390452277b6bd9a295758ecd6687f4950cf44de387cfeac9e7b10"),
        None,
    ),
    entry(
        "mosvar",
        "mosvar",
        "35c3b458bfed2ae2605c81b7455f0969cd2e7b1daacfb76d643c47e26fac9919",
        "9eb0045e27593043fed2c7f6457aed9427e7c81a0663384e0ae74f8987e15fe9",
        "4ea5098799b18d6f7cdda811135ef3c71da0b70dbe41e92050de1ce9ab236613",
        None,
        Some("608adc817742eb82a59963be189b34a0b89e8e64a1876e638f65222869161ef2"),
        None,
    ),
    entry(
        "mvsg_cmc",
        "mvsg_cmc",
        "e58386daa6f0c6433bbe10860b3a5795b788363f40afbbe2b2d81197b33e6e1e",
        "eb6530b0ec42df78ef0a8c304e1d0794b917874a5f803fb3049954912789a33e",
        "5cfeb6a8c135778c84ed73c1fb1193dc9cfebf21beceb3b9587a82e15cb09836",
        None,
        Some("1f516d901d48b3a84bc2f7d8ca0cbc64d7400af71a6cda486dda81001c516c61"),
        None,
    ),
    entry(
        "PSP104TVA",
        "PSP104TVA",
        "25a6fc32567fa9aa14bee399046f875bd46c4a6d2e92d757b9de5414872c26fd",
        "837bb31d7fe0ea8446b635176c279cd77b0f5fdbabca7adbae1844a1c3983b33",
        "ab8c9a4b150a8e902ec0a3368fb30f1cdd6ccf3a7ae93c21fbe00c300111dd39",
        None,
        Some("a2de19fc68c0f371a11f31240d9af2564881c65cd21aa4ac70328e4ca4c892e9"),
        None,
    ),
    entry(
        "PSP104VA",
        "PSP104VA",
        "427f4e2b2a666d51b78742e844057f5824428593dfae3f1c317b7a4f2228d3d1",
        "a45d1e909473c1f7f47598d91bab5797586fa45eb6dee49f3a3b26ab7dae98dd",
        "202c24ef116f974256a59b3ac61e0293fcab27d58d4764f27a1585fca5c1332e",
        None,
        Some("effb2f6b8f56020975d49e2cf1b0f9954793af8ad99c72488d93208a0d948b78"),
        None,
    ),
    entry(
        "PSPNQS104VA",
        "PSPNQS104VA",
        "f0de545f33cbf102fc8c92c97120bd2135bd4c98e44734c79b36c09177afe2d0",
        "322f54f8a0f62bb5140c8da21b1da20b6792e92608820fb75f288de795694cc9",
        "13f9c4f2d387550e4ce05689c4975018a8ac751aebbe18c36948b0881b541d94",
        None,
        Some("2d14ec8d4d14514152f0e17a82cce9948dfc58f12eb4b13d385a0a8c16c2ed08"),
        None,
    ),
    entry(
        "r2_cmc",
        "r2_cmc",
        "9f894d2514d714b4c248318d0ec11b0199868a86ff6b1c0bb8c66218896022d6",
        "61bbb88679ec37ec6ae464e72bb91076630938ef0a202ec617a25be5bd7e6b61",
        "dfbdcf3760d1534ce8353748bbeac16659b8f32d60238983261e4d0549929a2d",
        None,
        Some("9e04754fdf3136be3047e785acf7ad400fef09c6526e976bf90d2b00e11028d3"),
        None,
    ),
    entry(
        "r2_et_cmc",
        "r2_et_cmc",
        "6620f525bc7384c32b127b2c506424f1873f9bb0367eec5f5a5db5c0c53cfa63",
        "18542f7f017bd70ce26a12778bf8adc3cc6a942f83617d040158f135a3c6c302",
        "41ce13824a3ebb473d70f7a73760f6f9fc2796c4c5d04d3087b5062323885971",
        None,
        Some("97ad86096f66eab70e3cadfbcc5fc9d7d7bb91b270741a2400a42896c6c8433f"),
        None,
    ),
    entry(
        "r3_cmc",
        "r3_cmc",
        "42e1ac5876913e55942cdc598f1b09f29ee655dfe7cc51cb4a1b83edfb8bb390",
        "f2875cf3d859cc0353a9beff2bcb00579933bca728584542bbd67cfe5fa95b82",
        "80b9ad35570022c931de0e81fc0f8b497368076a482686a3b2fe5304935c52a5",
        None,
        Some("b6270d14a650f9cdb28bb001d48f42631d25600d66987ed3384106fd117a35b1"),
        None,
    ),
    entry(
        "vbic_4T_et_cf",
        "vbic_4T_et_cf",
        "d8330fe64713c0abc404be1cd7e9bc2678e821c93a5268c0552e017e961c723f",
        "da40e32e06e862349b39f32c494ec8109d24e21e4f670684b696d74165bafb31",
        "4b8754a39bd768b8aaad43f7f6d97c38a7a1eb562609b57019ff60d765cbff8c",
        None,
        Some("632a77af11e627b7fc958a5cfa764d204b133f9f4bf13336749fd0f32861bb0f"),
        None,
    ),
    entry(
        "vbic13",
        "vbic13",
        "9aa27a36b6c1eb4e70f914f5fa012694cb0eaa26fd21bb11ca0c01618d3a764d",
        "1c950f1c0ae955382f550740b2be24f583b4978276cf67107be777a13273559e",
        "9312813f56a56e981aabbc14ea83608659ac81ac770358376a7a8c95c6dd34be",
        Some("5aa31be810d1a88c93158583264a3c23af1fac052319b7f2bf2cd83e9e246dba"),
        Some("4408ff7da797cc591148f3e1e447035800ee3b794426762068d4785a9d3a8488"),
        Some("e169ac7dc9c1e7c7aa1a89ae67f7a49d30a0c08adaae0b897e4b4caa8efc3286"),
    ),
    entry(
        "vbic13_3t_et",
        "vbic13_3t_et",
        "66bf76297ac764e2c4d7502b2839427109d0525e26cfdb4c35c2d43a48a92e0b",
        "f340aab4612c692c5d1ea26765c9ae9c0a0103b72c82ee12915488122c9b1b4e",
        "7e88de59fbdce167496d1e2191a4c155decdabb3541052e36f03ff6b7662e495",
        None,
        Some("3395711ee2f216e4fb3d1c8f06a7ae0460f950cbd621940bed9edf893175837f"),
        None,
    ),
    entry(
        "vbic13_4t",
        "vbic13_4t",
        "8040736b52830a81e558339e0ecf145ade73abd6c8b03d6d9cd43e2b3c9c0725",
        "dddcb493e4c65f6588734735bc61fbc8a648d685caeca89a907a04023cfc8013",
        "0e388927d0bc8c4527e068e2575cdb8a02540f57d3e4af4c0094dc21795575de",
        None,
        Some("1c17acb1037e247accaa5a11ecb026c9322ec9cd311b81cbd8d4184e44bd8fbe"),
        None,
    ),
];

/// Exact combined checkpoint identities written by format 27 before format 28
/// split generated-model provenance into independent source, semantic, and
/// accepted-state-shape identities.
///
/// These aliases are deliberately separate from the format-26 identities in
/// the target catalog: the event-scheduling generator release changed every
/// combined artifact identity while retaining the same compatible accepted
/// state. Callers must select aliases using the checkpoint format version.
pub const GENERATED_VERILOGA_V27_COMBINED_IDENTITY_ALIASES: &[(&str, &str)] = &[
    (
        "angelov",
        "7195a85e146df9ec23b4a87da2e67703c5f5879df42556c7e894f4b76c919361",
    ),
    (
        "angelov_gan",
        "1bc380b2d47c3380c160d796d01010bcda9bcd7cd19b1c0aeb064be4931f797e",
    ),
    (
        "asmesd",
        "1167a02121d50dcde71ecda7fb4b2abe6670090952bf5d2fb783d1a6fad174ce",
    ),
    (
        "asmesd_dio",
        "ed6936e9c15adb818f9b99adf50d44f48c91e82047d76d96e79f473ae25be9a6",
    ),
    (
        "asmhemt",
        "3ecc0ec88e2462d94b3a16e2152a571e7e43954b9d24de0c897fe06783553cba",
    ),
    (
        "bjt505_va",
        "e8f73e8200377163b00917efd6985f7256487c6f12a6fafdb109a7feb08362d7",
    ),
    (
        "bjt505t_va",
        "79749bd50de3854308ad0578b321d1facfa92d359fac9f1cad3bbd8f6625cb19",
    ),
    (
        "bjtd505_va",
        "1c30eb712f5d906d3f55a2220909c9346ff902e127a09366ace56c0b3add953d",
    ),
    (
        "bjtd505t_va",
        "7a35963cface35789b644ce275b6632018c963255306cfafe661e6ff77b51ac9",
    ),
    (
        "bsimbulk",
        "221368970643265a466f5902717cc821acdc8ca73df93587c8253f2f567c5fb0",
    ),
    (
        "bsimcmg_va",
        "f290c56323f26c4fa55fd0e24a4a5385d6ddd56f89de296fa1be328fdcb7651a",
    ),
    (
        "bsimimg",
        "905c327dda746aba46ab790b7b7f5e0e0c5ac0041f965a49e1aaaeebd970c624",
    ),
    (
        "bsimsoi__18c250bc",
        "1ed131a6f89d92cfaa796451a4c7c2346e160f42fa99c10923093de4f17c4962",
    ),
    (
        "bsimsoi__e2aff994",
        "5a3bf3d7686e4e8fdb49632f6e48fc0f8c046d1732f4d3f6c570e82afa5baed9",
    ),
    (
        "bsimsoi_va",
        "5f1e88f1e4a5d6c575fb728bc9ed0f618972999cb8caadf5ccc14e0bf3d8a20c",
    ),
    (
        "DIODE_CMC",
        "cc7312e235a2bee7c2231c1d52120cbea82e2625f9d481abba9cd183a802d030",
    ),
    (
        "ekv_va",
        "e4f1c3dc414630970d57ed87c9382585d34a42f0c6c233147b7246cb1210e21d",
    ),
    (
        "ekv3_rf",
        "c552b81751ce6a5d48c44549244c07fef7f851c2ee5d93d3ca24dac48ccd59a6",
    ),
    (
        "EPFL_HEMT_10a",
        "dba155e89682aa3e6b832691ae3018fddaf7f865aaa024c23159096b54edf222",
    ),
    (
        "hicumL0va",
        "8a303c612617564899b5b5eee3757accd2cb1cf2541d2a5b0a3b776029e24fc3",
    ),
    (
        "hicumL2va",
        "bf5aedeaeddd94457e2e59842d63b4179a56dc9b994af506820bdb6894c59716",
    ),
    (
        "hisimhv_n4_va",
        "86d4f1f372fb92c217181b0abd1a988f17c51358d0de612020b92cb1044816c4",
    ),
    (
        "hisimhv_n5_va",
        "42efdf59c397a5392b7c298827d2d39d77b6abc439c412e5806f7a2c6043c865",
    ),
    (
        "hisimhv_va",
        "4a4b933d21b2b3abfda33d425ef5cbef216128e8e5c948af223d7870a127e6ac",
    ),
    (
        "hisimsoi_va__242bc21d",
        "3a4a7a22720efc95628c5274a3dd2df94249275f5e04836ffffe12620c6a552e",
    ),
    (
        "hisimsoi_va__38074d06",
        "0abb4d63f82c6aa030ff3149ec066ea132a96c812be74a341ee96f811cf81e51",
    ),
    (
        "hisimsoi_va__5be18005",
        "250efecd97386b8e46119f7e6ec7b78c4d7c75df380205cc8884315473d43a20",
    ),
    (
        "hisimsotb_va",
        "1ffa6c7a9cf3ce544c93bae9e9227b646046fb18f11577d18844f17cf1f3a917",
    ),
    (
        "JUNCAP200",
        "8220c95f6d2bd9395bc755913a1c1246f212533ed937573b59ee2276a8b539bd",
    ),
    (
        "l_utsoi__485e0ac9",
        "80e3f2a6eb8f34f16cfe35eba5aa35fbc99a12ed8bbe9413d6a96c06d55bd0f3",
    ),
    (
        "l_utsoi__832ce87d",
        "08dcda83a601506ccec28cc6347910c7f113f4aa9dfc04bf10aad68337c082d3",
    ),
    (
        "mosvar",
        "54ba5901b913acc16f519fbb56256556b1d3b7326e3060f921c5d7a227546663",
    ),
    (
        "mvsg_cmc",
        "06a102a4ea64481f44582429a69cb0b157b945e4ab520ff90a2afe762ca34f7e",
    ),
    (
        "PSP104TVA",
        "259751beb1225eb09e9adb7f5ea2b4bcd23d34a33f12fc392b7bb97a01f0fe61",
    ),
    (
        "PSP104VA",
        "d16f78193b17b7bec269c69ccd1a53493738271e4a11a58697e0aa438c876974",
    ),
    (
        "PSPNQS104VA",
        "34206e57155b97d7c1ed96c5ab6368279cc5e750e693346a3b1d4e4c2d980fc1",
    ),
    (
        "r2_cmc",
        "1f1802025f8516cfa7b176323dde0e7815319d236ae7012cbf20f2327455c078",
    ),
    (
        "r2_et_cmc",
        "c1ea25531e8744a27a4b3a31b5cb4801ff94cff4c119c4a8af50ce0372223798",
    ),
    (
        "r3_cmc",
        "eb8a53f0b77b09bb756aa461a9ba17bf0be3504c6b27503541f478d67ffca226",
    ),
    (
        "vbic_4T_et_cf",
        "c96d1544161530e8eee86926dd1f189e1526ad437a2702820415d892ab941442",
    ),
    (
        "vbic13",
        "1c950f1c0ae955382f550740b2be24f583b4978276cf67107be777a13273559e",
    ),
    (
        "vbic13_3t_et",
        "ec583904fe2874b777bf4b9d11e838c1c8c223e0541f8b30d6b8c8f8930a145d",
    ),
    (
        "vbic13_4t",
        "ec64547d804d600a51904ec6fe1ffc5f28f7a02df137c4a7559f89bccf55faaf",
    ),
];

pub fn generated_veriloga_compatibility_entry(
    module_name: &str,
    source_identity: &str,
) -> Result<Option<&'static GeneratedVerilogACompatibilityCatalogEntry>, String> {
    validate_generated_veriloga_compatibility_catalog()?;
    let mut matches = GENERATED_VERILOGA_COMPATIBILITY_CATALOG
        .iter()
        .filter(|entry| {
            entry.module_name == module_name && entry.source_identity == source_identity
        });
    let matched = matches.next();
    if matches.next().is_some() {
        return Err("generated Verilog-A compatibility source key is ambiguous".to_string());
    }
    Ok(matched)
}

pub fn generated_veriloga_v26_compatibility_entry(
    public_model_name: &str,
    combined_identity: &str,
) -> Result<Option<&'static GeneratedVerilogACompatibilityCatalogEntry>, String> {
    validate_generated_veriloga_compatibility_catalog()?;
    let mut matches = GENERATED_VERILOGA_COMPATIBILITY_CATALOG
        .iter()
        .filter(|entry| {
            entry.public_model_name == public_model_name
                && entry.wire_v26_combined_identity_alias == Some(combined_identity)
        });
    let matched = matches.next();
    if matches.next().is_some() {
        return Err("generated Verilog-A compatibility v26 alias is ambiguous".to_string());
    }
    Ok(matched)
}

pub fn generated_veriloga_checkpoint_compatibility_entry(
    checkpoint_format_version: u32,
    public_model_name: &str,
    combined_identity: &str,
) -> Result<Option<&'static GeneratedVerilogACompatibilityCatalogEntry>, String> {
    validate_generated_veriloga_compatibility_catalog()?;
    match checkpoint_format_version {
        26 => generated_veriloga_v26_compatibility_entry(public_model_name, combined_identity),
        27 => {
            let mut aliases = GENERATED_VERILOGA_V27_COMBINED_IDENTITY_ALIASES
                .iter()
                .filter(|(model, identity)| {
                    *model == public_model_name && *identity == combined_identity
                });
            let matched = aliases.next();
            if aliases.next().is_some() {
                return Err("generated Verilog-A compatibility v27 alias is ambiguous".to_string());
            }
            let Some((model_name, _)) = matched else {
                return Ok(None);
            };
            unique_catalog_entry_by_public_model(model_name)
        }
        _ => Ok(None),
    }
}

/// Resolve a schema-v1 UI binding whose wire contract did not persist its
/// enclosing checkpoint format. Every accepted identity must be present in one
/// of the exact published legacy-format tables and map to one current target.
pub fn generated_veriloga_wire_compatibility_entry(
    public_model_name: &str,
    combined_identity: &str,
) -> Result<Option<&'static GeneratedVerilogACompatibilityCatalogEntry>, String> {
    validate_generated_veriloga_compatibility_catalog()?;
    let v26 = GENERATED_VERILOGA_COMPATIBILITY_CATALOG
        .iter()
        .find(|entry| {
            entry.public_model_name == public_model_name
                && entry.wire_v26_combined_identity_alias == Some(combined_identity)
        });
    let v27 = GENERATED_VERILOGA_V27_COMBINED_IDENTITY_ALIASES
        .iter()
        .any(|(model, identity)| *model == public_model_name && *identity == combined_identity);
    match (v26, v27) {
        (Some(entry), _) => Ok(Some(entry)),
        (None, true) => unique_catalog_entry_by_public_model(public_model_name),
        (None, false) => Ok(None),
    }
}

fn unique_catalog_entry_by_public_model(
    public_model_name: &str,
) -> Result<Option<&'static GeneratedVerilogACompatibilityCatalogEntry>, String> {
    let mut entries = GENERATED_VERILOGA_COMPATIBILITY_CATALOG
        .iter()
        .filter(|entry| entry.public_model_name == public_model_name);
    let matched = entries.next();
    if entries.next().is_some() {
        return Err("generated Verilog-A compatibility public-model key is ambiguous".to_string());
    }
    Ok(matched)
}

pub fn validate_generated_veriloga_compatibility_catalog() -> Result<(), String> {
    for (index, entry) in GENERATED_VERILOGA_COMPATIBILITY_CATALOG.iter().enumerate() {
        if entry.module_name.is_empty() || entry.public_model_name.is_empty() {
            return Err(format!(
                "generated compatibility entry {index} has an empty model key"
            ));
        }
        if entry.target_descriptor_abi_version == 0 {
            return Err(format!(
                "generated compatibility entry {index} has descriptor ABI zero"
            ));
        }
        for (label, value) in [
            ("source identity", entry.source_identity),
            ("semantic identity", entry.semantic_identity),
            (
                "accepted-state shape identity",
                entry.accepted_state_shape_identity,
            ),
        ] {
            if !is_lower_hex_digest(value) {
                return Err(format!(
                    "generated compatibility entry {index} has invalid {label}"
                ));
            }
        }
        for (label, value) in [
            (
                "semantic identity override artifact",
                entry.semantic_identity_override_artifact,
            ),
            (
                "v26 combined identity alias",
                entry.wire_v26_combined_identity_alias,
            ),
            (
                "UI-v1 descriptor signature alias",
                entry.wire_ui_v1_descriptor_signature_alias,
            ),
        ] {
            if value.is_some_and(|value| !is_lower_hex_digest(value)) {
                return Err(format!(
                    "generated compatibility entry {index} has invalid {label}"
                ));
            }
        }
    }

    for (left_index, left) in GENERATED_VERILOGA_COMPATIBILITY_CATALOG.iter().enumerate() {
        for right in &GENERATED_VERILOGA_COMPATIBILITY_CATALOG[left_index + 1..] {
            if left.module_name == right.module_name
                && left.source_identity == right.source_identity
            {
                return Err("generated compatibility catalog has a duplicate source key".into());
            }
            if let (Some(left_alias), Some(right_alias)) = (
                left.wire_v26_combined_identity_alias,
                right.wire_v26_combined_identity_alias,
            ) && left.public_model_name == right.public_model_name
                && left_alias == right_alias
            {
                return Err("generated compatibility catalog has a duplicate v26 alias".into());
            }
        }
    }

    if GENERATED_VERILOGA_V27_COMBINED_IDENTITY_ALIASES.len()
        != GENERATED_VERILOGA_COMPATIBILITY_CATALOG.len()
    {
        return Err("generated compatibility v27 alias census is incomplete".into());
    }
    for (index, (public_model_name, combined_identity)) in
        GENERATED_VERILOGA_V27_COMBINED_IDENTITY_ALIASES
            .iter()
            .enumerate()
    {
        if public_model_name.is_empty() || !is_lower_hex_digest(combined_identity) {
            return Err(format!(
                "generated compatibility v27 alias {index} is malformed"
            ));
        }
        let matching_targets = GENERATED_VERILOGA_COMPATIBILITY_CATALOG
            .iter()
            .filter(|entry| entry.public_model_name == *public_model_name)
            .count();
        if matching_targets != 1 {
            return Err(format!(
                "generated compatibility v27 alias {index} has {matching_targets} target entries"
            ));
        }
        for (other_model, _) in &GENERATED_VERILOGA_V27_COMBINED_IDENTITY_ALIASES[index + 1..] {
            if public_model_name == other_model {
                return Err("generated compatibility v27 alias table has a duplicate model".into());
            }
        }
    }
    Ok(())
}

fn is_lower_hex_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

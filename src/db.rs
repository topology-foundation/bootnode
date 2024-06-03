use libp2p::identity;
use std::path::PathBuf;

/// RocksDB data path
const DB_PATH: &str = "./boot_db";

/// Coluumn family name for storing node's private key
const PK_CF: &str = "pk";

/// Storage key used for storing p2p private key
const RAMD_P2P_KEYPAIR_KEY: &[u8] = "ramd_p2p_pk".as_bytes();

pub struct RocksStorage {
    db: rocksdb::DB,
}

impl RocksStorage {
    pub fn new() -> eyre::Result<Self> {
        let path: PathBuf = DB_PATH.into();

        let mut rocks_options = rocksdb::Options::default();
        rocks_options.create_if_missing(true);
        rocks_options.create_missing_column_families(true);

        let cf_names = vec![PK_CF];
        let cfs = cf_names
            .clone()
            .iter()
            .map(|cf| rocksdb::ColumnFamilyDescriptor::new(*cf, rocks_options.clone()))
            .collect::<Vec<rocksdb::ColumnFamilyDescriptor>>();

        let db = rocksdb::DB::open_cf_descriptors(&rocks_options, path, cfs)?;

        // verify column family is accessible
        for cf_name in cf_names.iter() {
            let _ = db
                .cf_handle(cf_name)
                .unwrap_or_else(|| panic!("{cf_name} column family must be created"));
        }

        Ok(Self { db })
    }

    /// Retrieves private key from storage. Key expected to be stored in protobuf encoded format
    pub fn get_node_pk_opt(&self) -> eyre::Result<Option<identity::Keypair>> {
        let cf = self.cf_handle(PK_CF);

        let pk = self.db.get_cf(cf, RAMD_P2P_KEYPAIR_KEY)?;
        if let Some(key) = pk {
            Ok(Some(identity::Keypair::from_protobuf_encoding(&key)?))
        } else {
            Ok(None)
        }
    }

    /// Saves private key. Private key is encoded using protobuf encoding
    pub fn store_node_pk(&self, pk: &identity::Keypair) -> eyre::Result<()> {
        let cf = self.cf_handle(PK_CF);
        self.db
            .put_cf(cf, RAMD_P2P_KEYPAIR_KEY, pk.to_protobuf_encoding()?)?;

        Ok(())
    }

    fn cf_handle(&self, cf: &str) -> &rocksdb::ColumnFamily {
        let handle = self.db.cf_handle(cf).unwrap();
        handle
    }
}

// RocksDB implements Send + Sync.
unsafe impl Send for RocksStorage {}
unsafe impl Sync for RocksStorage {}

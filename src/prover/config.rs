use crate::{
    define_enum,
    fri::{DefaultFriComputer, FriComputer},
};
use clap::Args;
use serde::{Deserialize, Serialize};
use serde_json::Result;

define_enum! {
    CommitmentHash,
    keccak256_masked160_lsb => "keccak256_masked160_lsb",
    blake256_masked248_lsb => "blake256_masked248_lsb",
}

define_enum! {
    PageHash,
    pedersen => "pedersen",
    keccak256 => "keccak256",
}

define_enum! {
    Hash,
    poseidon3 => "poseidon3",
    blake256 => "blake256",
    keccak256 => "keccak256",
}

define_enum! {
    PowHash,
    blake256 => "blake256",
    keccak256 => "keccak256",
}

#[derive(Args, Serialize, Deserialize, Debug, Clone)]
pub struct StatementParameters {
    #[clap(long = "page_hash", default_value = "pedersen", value_enum)]
    page_hash: Option<PageHash>,
}

#[derive(Args, Serialize, Deserialize, Debug, Clone)]
pub struct StarkParameters {
    #[clap(flatten)]
    pub fri: FriParameters,
    #[clap(long = "log_n_cosets", default_value = "2")]
    pub log_n_cosets: Option<i32>,
}

#[derive(Args, Serialize, Deserialize, Debug, Clone)]
pub struct FriParameters {
    #[clap(long = "fri_step_list", num_args = 1.., value_delimiter = ' ', help = "autogenerated if not explicitly provided")]
    pub fri_step_list: Option<Vec<u32>>,
    #[clap(long = "last_layer_degree_bound", default_value = "64")]
    pub last_layer_degree_bound: Option<u32>,
    #[clap(long = "n_queries", default_value = "16")]
    pub n_queries: Option<u32>,
    #[clap(long = "proof_of_work_bits", default_value = "32")]
    pub proof_of_work_bits: Option<u32>,
}

#[derive(Args, Serialize, Deserialize, Debug, Clone)]
pub struct ProverParametersConfig {
    #[clap(long = "field", default_value = "PrimeField0")]
    field: Option<String>,
    #[clap(long = "channel_hash", default_value = "poseidon3", value_enum)]
    channel_hash: Option<Hash>,
    #[clap(
        long = "commitment_hash",
        default_value = "keccak256-masked160-lsb",
        value_enum
    )]
    commitment_hash: Option<CommitmentHash>,
    #[clap(long = "n_verifier_friendly_commitment_layers", default_value = "9999")]
    n_verifier_friendly_commitment_layers: Option<u32>,
    #[clap(long = "pow_hash", default_value = "keccak256", value_enum)]
    pow_hash: Option<PowHash>,
    #[clap(flatten)]
    statement: StatementParameters,
    #[clap(flatten)]
    stark: StarkParameters,
    #[clap(long = "use_extension_field", default_value = "false")]
    use_extension_field: Option<bool>,
    #[clap(long = "verifier_friendly_channel_updates", default_value = "true")]
    verifier_friendly_channel_updates: Option<bool>,
    #[clap(
        long = "verifier_friendly_commitment_hash",
        default_value = "poseidon3",
        value_enum
    )]
    verifier_friendly_commitment_hash: Option<Hash>,
}

impl ProverParametersConfig {
    pub fn new(
        nb_steps: u32,
        parameter_config: &ProverParametersConfig,
    ) -> Result<ProverParametersConfig> {
        let computed_fri_parameters =
            FriParameters::from(DefaultFriComputer.compute_fri_parameters(nb_steps));
        let computed_fri_step_list = parameter_config
            .stark
            .fri
            .fri_step_list
            .clone()
            .unwrap_or(computed_fri_parameters.fri_step_list.unwrap());

        let prover_parameters = ProverParametersConfig {
            field: parameter_config.field.clone(),
            channel_hash: parameter_config.channel_hash.clone(),
            commitment_hash: parameter_config.commitment_hash.clone(),
            n_verifier_friendly_commitment_layers: parameter_config
                .n_verifier_friendly_commitment_layers,
            pow_hash: parameter_config.pow_hash.clone(),
            statement: StatementParameters {
                page_hash: parameter_config.statement.page_hash.clone(),
            },
            stark: StarkParameters {
                fri: FriParameters {
                    fri_step_list: Some(computed_fri_step_list),
                    last_layer_degree_bound: parameter_config.stark.fri.last_layer_degree_bound,
                    n_queries: parameter_config.stark.fri.n_queries,
                    proof_of_work_bits: parameter_config.stark.fri.proof_of_work_bits,
                },
                log_n_cosets: parameter_config.stark.log_n_cosets,
            },
            use_extension_field: parameter_config.use_extension_field,
            verifier_friendly_channel_updates: parameter_config.verifier_friendly_channel_updates,
            verifier_friendly_commitment_hash: parameter_config
                .verifier_friendly_commitment_hash
                .clone(),
        };

        Ok(prover_parameters)
    }
}

impl Default for ProverParametersConfig {
    fn default() -> Self {
        ProverParametersConfig {
            field: Some("PrimeField0".to_string()),
            channel_hash: Some(Hash::poseidon3),
            commitment_hash: Some(CommitmentHash::keccak256_masked160_lsb),
            n_verifier_friendly_commitment_layers: Some(0),
            pow_hash: Some(PowHash::keccak256),
            statement: StatementParameters {
                page_hash: Some(PageHash::pedersen),
            },
            stark: StarkParameters {
                fri: FriParameters {
                    fri_step_list: None,
                    last_layer_degree_bound: Some(64),
                    n_queries: Some(16),
                    proof_of_work_bits: Some(32),
                },
                log_n_cosets: Some(4),
            },
            use_extension_field: Some(false),
            verifier_friendly_channel_updates: Some(true),
            verifier_friendly_commitment_hash: Some(Hash::poseidon3),
        }
    }
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct CachedLdeConfig {
    #[clap(long = "store_full_lde", default_value = "false")]
    pub store_full_lde: Option<bool>,
    #[clap(long = "use_fft_for_eval", default_value = "false")]
    pub use_fft_for_eval: Option<bool>,
}

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct ProverConfig {
    #[clap(flatten)]
    pub cached_lde_config: CachedLdeConfig,
    #[clap(long = "constraint_polynomial_task_size", default_value = "256")]
    pub constraint_polynomial_task_size: Option<i32>,
    #[clap(long = "n_out_of_memory_merkle_layers", default_value = "1")]
    pub n_out_of_memory_merkle_layers: Option<i32>,
    #[clap(long = "table_prover_n_tasks_per_segment", default_value = "32")]
    pub table_prover_n_tasks_per_segment: Option<i32>,
}

impl ProverConfig {
    pub fn new(config: &ProverConfig) -> Result<ProverConfig> {
        Ok(ProverConfig {
            cached_lde_config: CachedLdeConfig {
                store_full_lde: config.cached_lde_config.store_full_lde,
                use_fft_for_eval: config.cached_lde_config.use_fft_for_eval,
            },
            constraint_polynomial_task_size: config.constraint_polynomial_task_size,
            n_out_of_memory_merkle_layers: config.n_out_of_memory_merkle_layers,
            table_prover_n_tasks_per_segment: config.table_prover_n_tasks_per_segment,
        })
    }
}

impl Default for ProverConfig {
    fn default() -> Self {
        ProverConfig {
            cached_lde_config: CachedLdeConfig {
                store_full_lde: Some(false),
                use_fft_for_eval: Some(false),
            },
            constraint_polynomial_task_size: Some(256),
            n_out_of_memory_merkle_layers: Some(1),
            table_prover_n_tasks_per_segment: Some(32),
        }
    }
}

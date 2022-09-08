use crate::fixture::Fixture;
use crate::generation::prototyping::{
    PrototypesLweCiphertextVector, PrototypesLweSecretKey, PrototypesLweSeededCiphertextVector,
    PrototypesPlaintextVector,
};
use crate::generation::synthesizing::{
    SynthesizesLweSecretKey, SynthesizesLweSeededCiphertextVector, SynthesizesPlaintextVector,
};
use crate::generation::{IntegerPrecision, KeyDistributionMarker, Maker};
use crate::raw::generation::RawUnsignedIntegers;
use crate::raw::statistical_test::assert_noise_distribution;
use concrete_core::prelude::{
    LweCiphertextCount, LweDimension, LweSecretKeyEntity,
    LweSeededCiphertextVectorEncryptionEngine, LweSeededCiphertextVectorEntity,
    PlaintextVectorEntity, Variance,
};

/// A fixture for the types implementing the `LweSeededCiphertextEncryptionEngine` trait.
pub struct LweSeededCiphertextVectorEncryptionFixture;

#[derive(Debug)]
pub struct LweSeededCiphertextVectorEncryptionParameters {
    pub noise: Variance,
    pub lwe_dimension: LweDimension,
    pub lwe_ciphertext_count: LweCiphertextCount,
}

impl<Precision, KeyDistribution, Engine, PlaintextVector, SecretKey, CiphertextVector>
    Fixture<Precision, (KeyDistribution,), Engine, (PlaintextVector, SecretKey, CiphertextVector)>
    for LweSeededCiphertextVectorEncryptionFixture
where
    Precision: IntegerPrecision,
    KeyDistribution: KeyDistributionMarker,
    Engine: LweSeededCiphertextVectorEncryptionEngine<SecretKey, PlaintextVector, CiphertextVector>,
    PlaintextVector: PlaintextVectorEntity,
    SecretKey: LweSecretKeyEntity,
    CiphertextVector: LweSeededCiphertextVectorEntity,
    Maker: SynthesizesPlaintextVector<Precision, PlaintextVector>
        + SynthesizesLweSecretKey<Precision, KeyDistribution, SecretKey>
        + SynthesizesLweSeededCiphertextVector<Precision, KeyDistribution, CiphertextVector>,
{
    type Parameters = LweSeededCiphertextVectorEncryptionParameters;
    type RepetitionPrototypes =
        (<Maker as PrototypesLweSecretKey<Precision, KeyDistribution>>::LweSecretKeyProto,);
    type SamplePrototypes =
        (<Maker as PrototypesPlaintextVector<Precision>>::PlaintextVectorProto,);
    type PreExecutionContext = (PlaintextVector, SecretKey);
    type PostExecutionContext = (PlaintextVector, SecretKey, CiphertextVector);
    type Criteria = (Variance,);
    type Outcome = (Vec<Precision::Raw>, Vec<Precision::Raw>);

    fn generate_parameters_iterator() -> Box<dyn Iterator<Item = Self::Parameters>> {
        Box::new(
            vec![
                LweSeededCiphertextVectorEncryptionParameters {
                    noise: Variance(0.00000001),
                    lwe_dimension: LweDimension(100),
                    lwe_ciphertext_count: LweCiphertextCount(1),
                },
                LweSeededCiphertextVectorEncryptionParameters {
                    noise: Variance(0.00000001),
                    lwe_dimension: LweDimension(100),
                    lwe_ciphertext_count: LweCiphertextCount(100),
                },
                LweSeededCiphertextVectorEncryptionParameters {
                    noise: Variance(0.00000001),
                    lwe_dimension: LweDimension(300),
                    lwe_ciphertext_count: LweCiphertextCount(100),
                },
                LweSeededCiphertextVectorEncryptionParameters {
                    noise: Variance(0.00000001),
                    lwe_dimension: LweDimension(600),
                    lwe_ciphertext_count: LweCiphertextCount(100),
                },
                LweSeededCiphertextVectorEncryptionParameters {
                    noise: Variance(0.00000001),
                    lwe_dimension: LweDimension(1000),
                    lwe_ciphertext_count: LweCiphertextCount(100),
                },
                LweSeededCiphertextVectorEncryptionParameters {
                    noise: Variance(0.00000001),
                    lwe_dimension: LweDimension(3000),
                    lwe_ciphertext_count: LweCiphertextCount(100),
                },
                LweSeededCiphertextVectorEncryptionParameters {
                    noise: Variance(0.00000001),
                    lwe_dimension: LweDimension(6000),
                    lwe_ciphertext_count: LweCiphertextCount(100),
                },
            ]
            .into_iter(),
        )
    }

    fn generate_random_repetition_prototypes(
        parameters: &Self::Parameters,
        maker: &mut Maker,
    ) -> Self::RepetitionPrototypes {
        let proto_secret_key = maker.new_lwe_secret_key(parameters.lwe_dimension);
        (proto_secret_key,)
    }

    fn generate_random_sample_prototypes(
        parameters: &Self::Parameters,
        maker: &mut Maker,
        _repetition_proto: &Self::RepetitionPrototypes,
    ) -> Self::SamplePrototypes {
        let raw_plaintext_vector = Precision::Raw::uniform_vec(parameters.lwe_ciphertext_count.0);
        let proto_plaintext_vector =
            maker.transform_raw_vec_to_plaintext_vector(raw_plaintext_vector.as_slice());
        (proto_plaintext_vector,)
    }

    fn prepare_context(
        _parameters: &Self::Parameters,
        maker: &mut Maker,
        repetition_proto: &Self::RepetitionPrototypes,
        sample_proto: &Self::SamplePrototypes,
    ) -> Self::PreExecutionContext {
        let (proto_secret_key,) = repetition_proto;
        let (proto_plaintext,) = sample_proto;
        let synth_plaintext_vector = maker.synthesize_plaintext_vector(proto_plaintext);
        let synth_secret_key = maker.synthesize_lwe_secret_key(proto_secret_key);
        (synth_plaintext_vector, synth_secret_key)
    }

    fn execute_engine(
        parameters: &Self::Parameters,
        engine: &mut Engine,
        context: Self::PreExecutionContext,
    ) -> Self::PostExecutionContext {
        let (plaintext_vector, secret_key) = context;
        let seeded_ciphertext_vector = unsafe {
            engine.encrypt_lwe_seeded_ciphertext_vector_unchecked(
                &secret_key,
                &plaintext_vector,
                parameters.noise,
            )
        };
        (plaintext_vector, secret_key, seeded_ciphertext_vector)
    }

    fn process_context(
        _parameters: &Self::Parameters,
        maker: &mut Maker,
        repetition_proto: &Self::RepetitionPrototypes,
        sample_proto: &Self::SamplePrototypes,
        context: Self::PostExecutionContext,
    ) -> Self::Outcome {
        let (plaintext_vector, secret_key, seeded_ciphertext_vector) = context;
        let (proto_secret_key,) = repetition_proto;
        let (input_proto_plaintext_vector,) = sample_proto;
        let proto_output_seeded_ciphertext_vector =
            maker.unsynthesize_lwe_seeded_ciphertext_vector(seeded_ciphertext_vector);
        let proto_output_ciphertext = maker
            .transform_lwe_seeded_ciphertext_vector_to_lwe_ciphertext_vector(
                &proto_output_seeded_ciphertext_vector,
            );
        maker.destroy_plaintext_vector(plaintext_vector);
        maker.destroy_lwe_secret_key(secret_key);
        let output_proto_plaintext_vector = maker
            .decrypt_lwe_ciphertext_vector_to_plaintext_vector(
                proto_secret_key,
                &proto_output_ciphertext,
            );
        (
            maker.transform_plaintext_vector_to_raw_vec(input_proto_plaintext_vector),
            maker.transform_plaintext_vector_to_raw_vec(&output_proto_plaintext_vector),
        )
    }

    fn compute_criteria(
        parameters: &Self::Parameters,
        _maker: &mut Maker,
        _repetition_proto: &Self::RepetitionPrototypes,
    ) -> Self::Criteria {
        (parameters.noise,)
    }

    fn verify(criteria: &Self::Criteria, outputs: &[Self::Outcome]) -> bool {
        let (means, actual): (Vec<_>, Vec<_>) = outputs.iter().cloned().unzip();
        let means: Vec<Precision::Raw> = means.into_iter().flatten().collect();
        let actual: Vec<Precision::Raw> = actual.into_iter().flatten().collect();
        assert_noise_distribution(actual.as_slice(), means.as_slice(), criteria.0)
    }
}

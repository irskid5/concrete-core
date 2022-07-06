use super::engine_error;
use crate::prelude::AbstractEngine;

use crate::specification::entities::{GlweCiphertextEntity, GlweSeededCiphertextEntity};

engine_error! {
    GlweSeededCiphertextToGlweCiphertextTransformationEngineError for GlweSeededCiphertextToGlweCiphertextTransformationEngine @
}

/// A trait for engines transforming GLWE seeded ciphertexts into GLWE ciphertexts.
///
/// # Semantics
///
/// This [pure](super#operation-semantics) operation moves the existing GLWE seeded ciphertext into
/// a GLWE ciphertext.
///
/// # Formal Definition
///
/// ## GLWE seeded ciphertext to GLWE ciphertext transformation
/// TODO
pub trait GlweSeededCiphertextToGlweCiphertextTransformationEngine<
    InputCiphertext,
    OutputCiphertext,
>: AbstractEngine where
    InputCiphertext: GlweSeededCiphertextEntity,
    OutputCiphertext: GlweCiphertextEntity<KeyDistribution = InputCiphertext::KeyDistribution>,
{
    /// Does the transformation of the GLWE seeded ciphertext into an GLWE ciphertext
    fn transform_glwe_seeded_ciphertext_to_glwe_ciphertext(
        &mut self,
        glwe_seeded_ciphertext: InputCiphertext,
    ) -> Result<
        OutputCiphertext,
        GlweSeededCiphertextToGlweCiphertextTransformationEngineError<Self::EngineError>,
    >;

    /// Unsafely transforms a GLWE seeded ciphertext into a GLWE ciphertext
    ///
    /// # Safety
    /// For the _general_ safety concerns regarding this operation, refer to the different variants
    /// of [`GlweSeededCiphertextToGlweCiphertextTransformationEngineError`].
    /// For safety concerns _specific_ to an engine, refer to the implementer safety section.
    unsafe fn transform_glwe_seeded_ciphertext_to_glwe_ciphertext_unchecked(
        &mut self,
        glwe_seeded_ciphertext: InputCiphertext,
    ) -> OutputCiphertext;
}
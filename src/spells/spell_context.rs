//The Struct represents the context for a spell and is used for casting spells.

pub const MAX_DISTANCE: i32 = 32;

pub enum InteractionHand {
    Left,
    Right,
}
pub struct SpellContext {
    /**
     * The player casting this spell.
     */
    pub caster: Entity,
    /**
     * The focal point of this spell. This can be the same as {@link #caster}, but will often be different,
     * like in cases where the spell is executed through a projectile bullet.
     */
    pub focal_point: Entity,

    /**
     * The compiled spell to execute.
     */
    pub compiled_spell: CompiledSpell,

    /**
     * The loopcast index of this context. This is always 0 when the spell is cast as not a
     * loopcast. Increments every time for each loopcast iteration.
     */
    pub loop_cast_index: i32,

    /**
     * Which hand the object containing this spell was cast from.
     * <p>
     * This is only used for loopcasting. If the context doesn't support loopcasting,
     * there is no need to set this field.
     */
    pub cast_from: InteractionHand,
}

#[derive(Debug)]
pub enum ArrowType {
    None,
    Out,
    In,
}

//

trait SpellParameter<'a, T: 'a>
where
    T: 'a,
{
    pub fn create(&self) -> T;
    pub fn create_with_arrow_type(&self, arrow_type: ArrowType) -> T;

    /**
     * checks if this parameter can accept the piece passed in. Default implementation
     * checks against {@link #getRequiredType()} and {@link #requiresConstant()}.
     */
    pub fn can_accept(&self, piece: SpellPiece) -> bool;

    /**
     * Checks if this parameter requires a constant ({@link EnumPieceType#CONSTANT}). Similarly to
     * {@link #getRequiredType()} this
     * is for internal use only.
     */
    fn requires_constant(&self) -> bool;

    /**
     * Gets the type that this parameter requires. This is evaluated against
     * {@link SpellPiece#getEvaluationType()}.<br>
     * If you want any type to be able to be accepted, use {@link Any}.class.<br>
     * This method is to only be used internally, and as such, is not required to be
     * implemented fully. For better control, use {@link #canAccept(SpellPiece)} and
     * override {@link #getRequiredTypeString()} for display.
     */

    pub fn get_required_type(&self) -> &'a str;
}

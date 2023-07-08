use bevy::prelude::*;

#[derive(Component)]
pub struct Personality {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
    pub freedom: f32,
}

impl Personality {
    pub fn new(
        openness: f32,
        conscientiousness: f32,
        extraversion: f32,
        agreeableness: f32,
        neuroticism: f32,
        freedom: f32,
    ) -> Self {
        Self {
            openness,
            conscientiousness,
            extraversion,
            agreeableness,
            neuroticism,
            freedom,
        }
    }
}

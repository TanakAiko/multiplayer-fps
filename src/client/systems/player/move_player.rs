use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct AccumulatedInput(pub Vec2);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PhysicalTranslation(pub Vec3);

#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
pub struct PreviousPhysicalTranslation(pub Vec3);

pub fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    // camera_query: Query<&Transform, With<WorldModelCamera>>,
    mut query: Query<(&mut AccumulatedInput, &mut Velocity, &Transform)>,
) {
    const SPEED: f32 = 8.;

    for (mut input, mut velocity, player_transform) in query.iter_mut() {
        // Récupérer la direction de la caméra
        // Extraire les directions horizontales
        let player_forward_horizontal = Vec3::new(
            player_transform.forward().x, 
            0.0, 
            player_transform.forward().z
        ).normalize();
        
        let player_right_horizontal = Vec3::new(
            player_transform.right().x, 
            0.0, 
            player_transform.right().z
        ).normalize();

        // Réinitialiser l'input
        *input = AccumulatedInput(Vec2::ZERO);

        // Mouvements directionnels basés sur l'orientation de la caméra
        if keyboard_input.pressed(KeyCode::KeyW) {
            input.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            input.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            input.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            input.x += 1.0;
        }

        // Calculer la direction de mouvement
        let move_direction =
            (player_forward_horizontal * input.y + player_right_horizontal * input.x).normalize_or_zero();

        // Appliquer la vitesse
        velocity.0 = move_direction * SPEED;
    }
}



// Avance la simulation physique à un pas de temps fixe
// Conserve l'historique de position pour l'interpolation
// Calcule le déplacement en fonction du temps écoulé
// Réinitialise l'input après utilisation

pub fn advance_physics(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut PhysicalTranslation,
        &mut PreviousPhysicalTranslation,
        &mut AccumulatedInput,
        &Velocity,
    )>,
) {
    for (
        mut current_physical_translation,
        mut previous_physical_translation,
        mut input,
        velocity,
    ) in query.iter_mut()
    {
        // Mémoriser la position précédente avant de mettre à jour la position actuelle
        // Cela permet de conserver l'état précédent pour l'interpolation
        previous_physical_translation.0 = current_physical_translation.0;

        // Mettre à jour la position physique 
        // - Utilise la vélocité multipliée par le temps écoulé depuis le dernier pas
        // - Permet un mouvement indépendant du framerate
        current_physical_translation.0 += velocity.0 * fixed_time.delta_secs();

        // Réinitialiser l'accumulateur d'input 
        // Une fois que l'input a été utilisé pour calculer le mouvement, on le réinitialise
        input.0 = Vec2::ZERO;
    }
}





// Gère l'interpolation entre deux positions physiques
// Assure un mouvement visuellement fluide
// Indépendant du framerate
// Utilise lerp (interpolation linéaire) pour lisser les mouvements
pub fn interpolate_rendered_transform(
    fixed_time: Res<Time<Fixed>>,
    mut query: Query<(
        &mut Transform,
        &PhysicalTranslation,
        &PreviousPhysicalTranslation,
    )>,
) {
    for (mut transform, current_physical_translation, previous_physical_translation) in
        query.iter_mut()
    {
        // Récupérer les positions physiques précédente et actuelle
        let previous = previous_physical_translation.0;
        let current = current_physical_translation.0;

        // Calculer la fraction de dépassement entre deux pas de temps fixes
        // alpha est une valeur entre 0 et 1 indiquant notre position exacte entre deux états
        let alpha = fixed_time.overstep_fraction();

        // Interpolation linéaire (lerp) entre la position précédente et actuelle
        // Crée un mouvement fluide et constant, indépendant du framerate
        let rendered_translation = previous.lerp(current, alpha);

        // Mettre à jour la transformation visuelle avec la position interpolée
        transform.translation = rendered_translation;
    }
}
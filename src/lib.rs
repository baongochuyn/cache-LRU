
pub mod cache;
// LRU CACHE
// CACHE --> Enregistrer la donnée en mémoire localement
// LRU --> Policy d'éviction des données
// LRU --> Least Recently Used --> Quand notre cache est plein, on éjecte la donnée qui n'a pas été utilisée depuis le plus longtemps
//
// INSERT A --> "value_A"
// INSERT B --> "value_B"
// INSERT C --> "value_C"
// INSERT D --> "value_D"
// READ B --> "value_B"
// INSERT E --> "value_E"
//
//
//
//
//
//
//
// ------- Cache (taille du cache: 3)
// D
// B
// E
//
//
pub use cache::{Cache, CacheLRU};
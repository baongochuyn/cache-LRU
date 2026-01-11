//! # cache-LRU
//!
//! Une bibliothèque de cache en mémoire pour Rust, générique et extensible, avec des politiques d'éviction modulables.
//! Par défaut, la politique est LRU (Least Recently Used) : quand le cache est plein, on enlève l'élément le moins utilisé récemment.
//!
//! ## Fonctionnalités
//! - Cache clé/valeur générique avec politique d'éviction personnalisable
//! - Politique LRU par défaut (supprime l'élément le moins récemment utilisé)
//! - Sauvegarde et chargement du cache dans un fichier
//! - Tests d'intégration et benchmarks avec Criterion
//! - Facile à étendre pour d'autres politiques (LFU, FIFO, etc.)
//!
//! ## Exemple
//! ```rust
//! use cache_lru::{Cache, CacheLRU};
//! let mut cache = Cache::<String, String>::new(3);
//! cache.put("A".to_string(), "valeur_A".to_string());
//! cache.put("B".to_string(), "valeur_B".to_string());
//! cache.put("C".to_string(), "valeur_C".to_string());
//! cache.get(&"A".to_string()); // Accède à A pour mettre à jour l'ordre LRU
//! cache.put("D".to_string(), "valeur_D".to_string()); // Éjecte B (le moins utilisé)
//! ```
//!
//! ## Exports
//! - [`Cache`] : Structure principale du cache, générique sur la clé, la valeur et la politique
//! - [`CacheLRU`] : Trait pour la politique d'éviction (par défaut : LRU)

pub mod cache;

pub use cache::{Cache, CacheLRU};

/// Cache module documentation
///
/// This module provides a generic key/value cache with a default Least Recently Used (LRU)
/// eviction policy. The cache size is configurable, and the module supports file persistence
/// for saving and loading cache state.
///
/// # Examples
///
/// ```rust
/// use cache_lru::{Cache, CacheLRU};
/// let mut cache = Cache::<String, String>::new(3);
/// cache.put("A".to_string(), "value_A".to_string());
/// cache.put("B".to_string(), "value_B".to_string());
/// cache.put("C".to_string(), "value_C".to_string());
/// assert_eq!(cache.get(&"A".to_string()), Some(&"value_A".to_string()));
/// cache.put("D".to_string(), "value_D".to_string()); // This will evict "B"
/// assert_eq!(cache.get(&"B".to_string()), None); // "B" has been evicted
/// ```

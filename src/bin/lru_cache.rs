use cache::cache::Cache;

fn main() {
    let mut cache = Cache::<String, String>::new_persistent(3, "cache_persistent.txt");

    cache.put("A".to_string(), "valeur_A".to_string());
    cache.put("B".to_string(), "valeur_B".to_string());
    cache.put("C".to_string(), "valeur_C".to_string());

    // Accéder à B pour mettre à jour l'ordre LRU
    let _ = cache.get(&"B".to_string());

    // Ajouter D, cela va éjecter l'élément le moins utilisé (A)
    cache.put("D".to_string(), "valeur_D".to_string());

    // Sauvegarder le cache dans un fichier
    cache.persist().expect("Erreur lors de la sauvegarde du cache");
    println!("\nCache sauvegardé dans 'cache_persistent.txt'.");
    // => C B D

    let cache2 = Cache::<String, String>::load_persistent(3, "cache_persistent.txt")
    .expect("Impossible de charger le cache");
}

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Definiamo il percorso del file in una costante o usiamo lazy_static 
// se vogliamo gestire il file in modo thread-safe.
const DEBUG_FILE_PATH: &str = "debug_log.txt";

#[macro_export]
macro_rules! debug_to_file {
    ($($arg:tt)*) => {
        {
        use std::io::Write;
        // Apertura del file in modalità Append (crea il file se non esiste)
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("debug_log.txt") // Puoi passare il path qui o usare una variabile
            .expect("Impossibile aprire il file di log");

        // Scrittura del contenuto formattato
        if let Err(e) = writeln!(file, $($arg)*) {
            eprintln!("Errore durante la scrittura nel file di log: {}", e);
        }
    }
};
}
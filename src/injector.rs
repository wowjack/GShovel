use std::collections::HashMap;

use libmem::*;

struct Injection {
    pproc: lm_process_t, // Parent process of the module
    module_name: String, // Name
    module: Option<lm_module_t>, // Module handle, None if not injected
    hooks: HashMap<String, Hook> // Map of hooks accessed by their names
}
impl Injection {
    pub fn new(target_process: &str, module_path: &str) -> Result<Self, String> {
        let pproc = LM_FindProcess(target_process).ok_or("Error finding target process.".to_string())?;
        Ok(Self {
            pproc,
            module_name: module_path.to_string(),
            module: None,
            hooks: HashMap::new()
        })
    }
    // Inject the module into the parent process if not already injected and set all defined hooks
    pub fn inject(&mut self) {

    }
    // Unset (not remove) hooks and eject the parent process
    pub fn eject(&mut self) {
        for hook in &mut self.hooks {
            //unset hook
        }
    }

    // The hook is set as soon as it is created
    pub fn set_hook(&mut self, from_symbol: &str, from_module: &str, to_symbol: &str) -> Result<(), String> {
        if let Some(Hook { active, .. }) = self.get_hook(from_symbol, from_module) {
            return Err("Hook already exists".to_string())
        }
        todo!()
        
        
    }
    //
    pub fn unset_hook() {

    }
    pub fn remove_hook(&mut self) {
        todo!()
    }
    pub fn get_hook(&self, from_symbol: &str, from_module: &str) -> Option<&Hook> {
        let hook_string = String::from_iter(from_module.chars().chain(".".chars()).chain(from_symbol.chars()));
        self.hooks.get(&hook_string)
    }

    /// Completely remove and delete all hooks and eject
    pub fn reset() {

    }

}


// The module is always defined since the Injector needs the process
// From address is always defined
// If the to module isn't injected, the trampoline will be invalid
struct Hook {
    from_module: lm_module_t,
    from_address: usize,
    trampoline: Option<(usize, usize)>,
    active: bool
}
impl Hook {
    fn new() -> Self {
        todo!()
    }
    fn hook() {

    }
    fn rehook() {

    }
}
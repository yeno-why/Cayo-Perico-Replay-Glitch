use windows_wfp::{ initialize_wfp, FilterEnumerator, Action, Direction, FilterBuilder, FilterRule, FilterWeight, IpAddrMask, WfpEngine, WfpResult };
use std::sync::atomic::{ AtomicBool, Ordering };
use win_hotkeys::HotkeyManager;
use win_hotkeys::VKey;
use std::sync::Mutex;

static ENABLED: AtomicBool = AtomicBool::new(false);
static FILTER: Mutex<Option<u64>> = Mutex::new(None);
static WFP_ENGINE: Mutex<Option<WfpEngine>> = Mutex::new(None);

fn main() -> WfpResult<()> {
    let engine: WfpEngine = WfpEngine::new()?;
    initialize_wfp(&engine)?;

    for f in FilterEnumerator::all(&engine)? {
        if f.name == "__Block__YENO_WHY__CPRG__" {
            let _ = FilterBuilder::delete_filter(&engine, f.id);
        }
    }

    *WFP_ENGINE.lock().unwrap() = Some(engine);

    let mut hkm: HotkeyManager<()> = HotkeyManager::new();
    hkm.register_hotkey(VKey::F12, &[VKey::from_keyname("ctrl").unwrap()], || {
        let engine_guard: std::sync::MutexGuard<'_, Option<WfpEngine>> = WFP_ENGINE.lock().unwrap();
        let engine: &WfpEngine = engine_guard.as_ref().expect("WFP engine not initialized");

        if ENABLED.load(Ordering::SeqCst) {
            if let Some(id) = FILTER.lock().unwrap().take() {
                let _ = FilterBuilder::delete_filter(engine, id);
            }
            ENABLED.store(false, Ordering::SeqCst);
            println!("off");
        } else {
            let rule: FilterRule = FilterRule::new("__Block__YENO_WHY__CPRG__", Direction::Outbound, Action::Block).with_weight(FilterWeight::UserBlock).with_remote_ip(IpAddrMask::from_cidr("192.81.241.171/32").unwrap());
            match FilterBuilder::add_filter(engine, &rule) {
                Ok(id) => {
                    *FILTER.lock().unwrap() = Some(id);
                    ENABLED.store(true, Ordering::SeqCst);
                    println!("on");
                }
                Err(e) => eprintln!("failed: {e}"),
            }
        }
    }).unwrap();

    hkm.event_loop();
    return Ok(());
}
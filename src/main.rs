extern crate alloc;

use core::time::Duration;
use core::panic::PanicInfo;
use libc_print::std_name::{println};
use edge_executor::Executor;
use ethercrab::{Client, ClientConfig, PduStorage, SlaveGroup, Timeouts};
use worst_executor::block_on;

use alloc::{sync::Arc, boxed::Box};

mod time;

const MAX_SLAVES: usize = 16;
const MAX_PDU_DATA: usize = 1100;
const MAX_FRAMES: usize = 16;
const PDI_LEN: usize = 64;

static PDU_STORAGE: PduStorage<MAX_FRAMES, MAX_PDU_DATA> = PduStorage::new();

async fn init() {
    let (tx, rx, pdu_loop) = PDU_STORAGE.try_split().expect("can only split once");

    let client = Arc::new(Client::new(
        pdu_loop,
        Timeouts {
            wait_loop_delay: Duration::from_millis(2),
            mailbox_response: Duration::from_millis(1000),
            ..Default::default()
        },
        ClientConfig::default(),
    ));

    let group = client
    .init_single_group::<MAX_SLAVES, PDI_LEN>(SlaveGroup::new(|slave| {
        Box::pin(async {
            if slave.name() == "EL3004" {
                println!("Found EL3004. Configuring...");

                // Taken from TwinCAT
                slave.sdo_write(0x1c12, 0, 0u8).await?;
                slave.sdo_write(0x1c13, 0, 0u8).await?;

                slave.sdo_write(0x1c13, 1, 0x1a00u16).await?;
                slave.sdo_write(0x1c13, 2, 0x1a02u16).await?;
                slave.sdo_write(0x1c13, 3, 0x1a04u16).await?;
                slave.sdo_write(0x1c13, 4, 0x1a06u16).await?;
                slave.sdo_write(0x1c13, 0, 4u8).await?;
            }

            Ok(())
        })
    }))
    .await
    .expect("Init");
}

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn main() {
    let e = Executor::new();
    e.spawn(async {
        init().await;
    });

    // block_on(async {
    //     init().await;
    // });
}
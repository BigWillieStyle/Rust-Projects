use sha2::{Digest, Sha256};
use std::fmt;
use std::thread;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};

// Define mining complexity
const DIFFICULTY: usize = 2;

/**
 * Block Structure
 */
struct Block {
    index: u32,            // A unique identifier for the block.
    timestamp: u64,        // The time when the block was created.
    data: String,          // The information stored in the block (e.g., transactions).
    previous_hash: String, // A reference to the hash of the previous block, ensuring the chain's integrity.
    hash: String,          // The hash of the current block, generated using its contents.
    nonce: u64,            // A number used in the mining process to find a valid hash.
}

/**
 * Method for creating new Block
 */
impl Block {
    fn new(index: u32, data: String, previous_hash: String) -> Block {
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        //let nonce = 0; // Initial nonce
        //let hash = calculate_hash(index, &previous_hash, timestamp, &data, nonce);
        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),
            nonce: 0,
        }
    }

    fn calculate_hash(&mut self) -> String {
        let data: String = format!(
            "{}{}{}{}{}",
            self.index, &self.previous_hash, self.timestamp, &self.data, self.nonce
        );

        let mut hasher = Sha256::new();
        hasher.update(data.as_bytes());
        let result = hasher.finalize();

        let hash_str = format!("{:x}", result);
        hash_str
    }

    // Method to mine the block with visual effects
    fn mine_block_with_visual_effects(&mut self) {
        let mut iterations = 0; // Initialize iterations counter
        loop {
            self.hash = self.calculate_hash(); // Calculate the hash of the block
            iterations += 1; // Increment iterations counter
            // Check if the hash meets the difficulty requirement
            if !self.hash.is_empty() && &self.hash[..DIFFICULTY] == "00".repeat(DIFFICULTY) {
                // Print a message indicating successful block mining
                println!("⛏️ Block mined: {}", self.index);
                break; // Exit the loop
            }
            // If the iterations exceed a certain limit, print the calculated hash and pause for visual effect
            if iterations > 100 {
                print!("⏳ Mining in progress... ");
                thread::sleep(Duration::from_millis(3000));
                println!("Calculated hash: {}", self.hash);
                break;
            }
            self.nonce += 1; // Increment the nonce for the next iteration
        }
    }
}

// Implementing formatting for Block structure to allow printing
impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let datetime = chrono::NaiveDateTime::from_timestamp(self.timestamp as i64, 0);
        write!(f, "Block {}: {} at {}", self.index, self.data, datetime)
    }
}

// Define the structure of the blockchain
struct Blockchain {
    chain: Vec<Block>, // Vector to hold blocks in the chain
}

impl Blockchain {
    // Constructor for creating a new blockchain with a genesis block
    fn new() -> Blockchain {
        let genesis_block = Block::new(0, String::new(), String::from("Genesis Block"));
        Blockchain {
            chain: vec![genesis_block], // Initialize chain with the genesis block
        }
    }

    // Method to add a new block to the blockchain
    fn add_block(&mut self, mut new_block: Block) {
        let previous_hash = self.chain.last().unwrap().hash.clone(); // Get hash of the previous block
        new_block.previous_hash = previous_hash; // Set the previous hash of the new block
        new_block.mine_block_with_visual_effects(); // Mine the new block
        self.chain.push(new_block); // Add the mined block to the chain
    }

    // Method to get the total number of blocks in the blockchain
    fn get_total_blocks(&self) -> usize {
        self.chain.len()
    }
}

// Main function for the blockchain simulation
fn main() {
    println!("🚀 Welcome to Blockchain Mining Simulator! 🚀");

    // Prompt user for miner name
    println!("👷 Enter your miner name: ");
    let mut miner_name = String::new();
    std::io::stdin()
        .read_line(&mut miner_name)
        .expect("Failed to read input");
    miner_name = miner_name.trim().to_string(); // Trim whitespace from input

    // Initialize a list of imaginary trader names
    let trader_names = vec![
        "Bob",
        "Linda",
        "John",
        "Larry",
        "David",
        "Renee",
        "Catherine",
        "Danny",
        "Kenny",
        "Daryl",
        "Anthony",
        "Chris",
        "George",
        "Kevin",
    ];

    // Initialize a new blockchain
    let mut block_token = Blockchain::new();

    // Start mining blocks and simulating transactions
    println!("\n⛏️  Let's start mining and simulating transactions!\n");
    let mut sender = miner_name.clone();
    for i in 0..trader_names.len() {
        println!("🧱 Mining block {}...⛏️", i + 1);
        let recipient = if i < trader_names.len() - 1 {
            trader_names[i + 1].to_string()
        } else {
            miner_name.clone() // Last transaction goes back to the miner
        };
        let transaction = format!("{} sent to {}", sender, recipient);
        let new_block = Block::new((i + 1) as u32, String::new(), transaction.clone());
        block_token.add_block(new_block);
        println!("✉️ Transaction: {}", transaction); // Display transaction
        sender = recipient; // Update sender for the next transaction
        println!();
    }

    // Calculate the total number of blocks added to the blockchain
    let total_blocks = block_token.get_total_blocks();
    println!("✅ Total blocks added to the blockchain: {}", total_blocks);

    // Calculate an arbitrary amount of Block Token traded (e.g., 10 Block Token per block)
    let block_token_per_block: usize = 137;
    let block_token_traded = total_blocks * block_token_per_block;
    println!(
        "💰 Total Block Token traded: {} Block Token",
        block_token_traded
    );

    // Display timestamp of simulation end
    let end_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let end_datetime = chrono::NaiveDateTime::from_timestamp(end_timestamp as i64, 0);
    println!("🕒 Simulation ended at: {}", end_datetime);

    // Print message when mining operation is completed
    println!("🎉 Congrats! Mining operation completed successfully!");
}

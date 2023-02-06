#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod Auction {

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Auction {
        /// The owner of the auction
        owner: AccountId,
        /// The NFT being auctioned
        nft: AccountId,
        /// The initial price of the NFT
        initial_price: Balance,
        /// The deadline of the auction
        deadline: Timestamp,
        /// The highest bid of the auction
        high_bid: Balance,
        /// The highest bidder of the auction
        high_bidder: AccountId,
        /// Whether the auction is completed
        completed: bool,
        /// Whether the auction is cancelled
        cancelled: bool,
    }

    #[ink(event)]
    pub struct NewAuction {
        nft: AccountId,
        initial_price: Balance,
        deadline: Timestamp,
    }

    #[ink(event)]
    pub struct BidPlaced {
        bid: Balance,
        bidder:
         AccountId,
    }

    impl Auction {
        #[ink(constructor)]
        pub fn new(nft: AccountId, initial_price: Balance, deadline: Timestamp) -> Self {
            let owner = Self::env().caller();
            Self {
                owner: owner,
                nft: nft,
                initial_price: initial_price,
                deadline: deadline,
                high_bid: initial_price,
                high_bidder: owner,
                completed: false,
                cancelled: false,
            }
        }
    
       #[ink(message)]
       pub fn place_bid(&mut self, bid: Balance) {
           // Check if the auction has been cancelled or completed
           assert!(!self.cancelled, "Auction has been cancelled");
           assert!(!self.completed, "Auction has already been completed");
           // Check if the auction deadline has passed
           assert!(self.env().block_timestamp() <= self.deadline, "Auction deadline has passed");
           // Check if the bid amount is greater than the current high bid
           assert!(bid > self.high_bid, "Bid must be higher than current high bid");
           // Deduct the bid amount from the caller's account
        //    self.env().transfer_from_ink_well(bid)?;
           // Transfer the current high bid deposit to the previous high bidder
           self.env().transfer(self.high_bidder, self.high_bid);
           // Update the high bid and high bidder information
           self.high_bid = bid;
           self.high_bidder = self.env().caller();
       }
    
        #[ink(message)]
        pub fn cancel_auction(&mut self) {
            // Check if the caller is the owner of the auction
            assert_eq!(self.owner, self.env().caller(), "Only the owner can cancel the auction");
            // Check if the auction has already been completed
            assert!(!self.completed, "Auction has already been completed");
            // Check if there are no bids placed yet
            assert!(self.high_bidder == self.owner, "Cannot cancel auction if there are bids placed");
            // Set the cancelled flag to true
            self.cancelled = true;
            // Transfer the NFT back to the owner
            // self.nft.transfer(self.owner);

            // This is not right, nft address != nft owner address
            self.nft = self.owner;
        }
    
        #[ink(message)]
        pub fn close_auction(&mut self) {
            // Check if the auction has already been completed
            assert!(!self.completed, "Auction has already been completed");
            // Check if the auction deadline has passed
            assert!(self.env().block_timestamp() >= self.deadline, "Auction has not ended yet");
            // Set the completed flag to true
            self.completed = true;
            // Check if the auction was cancelled
            if self.cancelled {
                // Transfer the NFT back to the owner
                // self.nft.transfer(self.owner);
                
                // This is not right, nft address != nft owner address
                self.nft = self.owner;
            } else {
                // Transfer the NFT to the highest bidder
                // self.nft.transfer(self.high_bidder);

                // This is not right, nft address != nft owner address
                self.nft = self.high_bidder;

                // Transfer the funds to the owner
                self.env().transfer(self.owner, self.high_bid);
            }
        }
    
        #[ink(message)]
        pub fn claim_nft(&mut self) {
            // Check if the auction has already been completed
            assert!(self.completed, "Auction has not completed yet");
            // Check if the caller is the high bidder
            assert_eq!(self.high_bidder, self.env().caller(), "You are not the winning bidder");
            // Transfer the NFT to the high bidder
            // self.nft.transfer(self.high_bidder);

            // This is not right, nft address != nft owner address
            self.nft = self.high_bidder;
        }
    }

}

use pinocchio::pubkey::Pubkey;
use super::Attributes;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemType {
    Weapon = 0,
    Armor = 1,
    Accessory = 2,
}

impl ItemType {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(ItemType::Weapon),
            1 => Some(ItemType::Armor),
            2 => Some(ItemType::Accessory),
            _ => None,
        }
    }
}

/// Item account - equipment that can be equipped to heroes
#[repr(C)]
pub struct Item {
    pub discriminator: u64,
    pub owner: Pubkey,           // Hero that owns this item
    pub item_type: u8,           // Weapon, Armor, or Accessory
    pub bonus_attributes: Attributes,
    pub created_at: i64,
}

impl Item {
    pub const DISCRIMINATOR: u64 = 0x6974656d_00000000; // "item" in hex

    pub fn new(owner: Pubkey, item_type: ItemType, bonus_attributes: Attributes, created_at: i64) -> Self {
        Self {
            discriminator: Self::DISCRIMINATOR,
            owner,
            item_type: item_type as u8,
            bonus_attributes,
            created_at,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.discriminator == Self::DISCRIMINATOR
    }

    pub fn get_item_type(&self) -> Option<ItemType> {
        ItemType::from_u8(self.item_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_initialization() {
        let owner = Pubkey::default();
        let attributes = Attributes::new_base();
        let item = Item::new(owner, ItemType::Weapon, attributes, 1234567890);
        
        assert!(item.is_initialized());
        assert_eq!(item.get_item_type(), Some(ItemType::Weapon));
    }
}

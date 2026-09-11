use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::plugins::wow::wotlk::realm::object::PackedGuid;

/// Lifecycle metadata for an object observed by [`super::ObjectProcessor`].
///
/// Timestamps are Unix milliseconds captured when Tentacli processes the
/// corresponding packet. This registry is intentionally separate from
/// [`super::Object`], which remains the reconstructed WotLK object state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectLifecycle {
    created_at: u64,
    updated_at: u64,
    removed_at: Option<u64>,
    removal_reason: Option<ObjectRemovalReason>,
}

impl ObjectLifecycle {
    #[inline]
    pub fn created_at(&self) -> u64 {
        self.created_at
    }

    #[inline]
    pub fn updated_at(&self) -> u64 {
        self.updated_at
    }

    #[inline]
    pub fn removed_at(&self) -> Option<u64> {
        self.removed_at
    }

    #[inline]
    pub fn removal_reason(&self) -> Option<ObjectRemovalReason> {
        self.removal_reason
    }

    #[inline]
    pub fn is_removed(&self) -> bool {
        self.removed_at.is_some()
    }

    #[inline]
    pub(crate) fn created(now: u64) -> Self {
        Self {
            created_at: now,
            updated_at: now,
            removed_at: None,
            removal_reason: None,
        }
    }

    #[inline]
    pub(crate) fn mark_updated(&mut self, now: u64) {
        self.updated_at = now;
    }

    #[inline]
    pub(crate) fn mark_removed(&mut self, now: u64, reason: ObjectRemovalReason) {
        if self.removed_at.is_none() {
            self.removed_at = Some(now);
            self.removal_reason = Some(reason);
        }
    }
}

/// Why an object stopped being part of the current [`super::ObjectMap`].
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectRemovalReason {
    Destroyed,
    OutOfRange,
}

/// Latest lifecycle information known for each observed object GUID.
///
/// Removed entries are deliberately retained so downstream consumers can
/// inspect when and why an object disappeared. A later create for the same
/// GUID replaces the entry and starts a new lifecycle.
pub type ObjectLifecycleRegistry = HashMap<PackedGuid, ObjectLifecycle>;

#[inline]
pub(crate) fn now_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_lifecycle_starts_active_with_matching_timestamps() {
        let lifecycle = ObjectLifecycle::created(123);

        assert_eq!(lifecycle.created_at(), 123);
        assert_eq!(lifecycle.updated_at(), 123);
        assert_eq!(lifecycle.removed_at(), None);
        assert_eq!(lifecycle.removal_reason(), None);
        assert!(!lifecycle.is_removed());
    }

    #[test]
    fn first_removal_is_stable_and_later_removals_do_not_rewrite_history() {
        let mut lifecycle = ObjectLifecycle::created(100);
        lifecycle.mark_updated(150);
        lifecycle.mark_removed(200, ObjectRemovalReason::OutOfRange);
        lifecycle.mark_removed(300, ObjectRemovalReason::Destroyed);

        assert_eq!(lifecycle.created_at(), 100);
        assert_eq!(lifecycle.updated_at(), 150);
        assert_eq!(lifecycle.removed_at(), Some(200));
        assert_eq!(
            lifecycle.removal_reason(),
            Some(ObjectRemovalReason::OutOfRange)
        );
        assert!(lifecycle.is_removed());
    }
}

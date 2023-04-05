pub struct UserCollection {
    users: [&'static str; 3],
}

impl UserCollection {
    pub fn new() -> Self {
        Self {
            users: ["Alice", "Bob", "Carl"],
        }
    }

    pub fn iter(&self) -> UserIterator {
        UserIterator {
            index: 0,
            user_collection: self,
        }
    }
}

pub struct UserIterator<'a> {
    index: usize,
    user_collection: &'a UserCollection,
}

impl Iterator for UserIterator<'_> {
    type Item = &'static str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.user_collection.users.len() {
            let user = Some(self.user_collection.users[self.index]);
            self.index += 1;
            return user;
        }

        None
    }
}

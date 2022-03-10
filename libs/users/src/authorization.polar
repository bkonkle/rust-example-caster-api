allow(actor, action, resource) if
  has_permission(actor, action, resource);

actor User {}

resource Profile {
    relations = {user: User};
}

allow(actor, action, resource) if
  has_permission(actor, action, resource);

has_role(user: User, name: String, resource: Resource) if
  role in user.profile.roles and
  role.name = name and
  role.resource = resource;

actor User {
    relations = {profile: Profile};
}

resource Profile {}

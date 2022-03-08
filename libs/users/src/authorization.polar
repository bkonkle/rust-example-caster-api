allow(actor, action, resource) if
  has_permission(actor, action, resource);

has_role(profile: Profile, name: String, resource: Resource) if
  role in profile.roles and
  role.name = name and
  role.resource = resource;

actor Profile {}

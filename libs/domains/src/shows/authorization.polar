has_role(user: User, role_key: String, show: Show) if
  role in user.roles and
  role.role_key = role_key and
  role.resource_table = "shows" and
  role.resource_id = show.id;

# Any logged-in user can create a new show.
has_permission(_: User, "create", _: Show);

resource Show {
    permissions = [
        # Update details about a Show
        "update",
        # Delete a Show
        "delete",
        # Create, update, and delete any Episodes for a Show
        "manage_episodes",
        # Grant or revoke Profile Roles for a Show
        "manage_roles"
    ];
    roles = [
        # Able to update a Show and manage Episodes
        "manager",
        # Able to fully control a Show
        "admin"
    ];

    "update" if "manager";
    "manage_episodes" if "manager";

    "delete" if "admin";
    "manage_roles" if "admin";
    "manager" if "admin";
}

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
        # Grant or revoke User Roles for a Show
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

resource Episode {
    permissions = [
        # Read chat Messages for an Episode
        "episode_read_chat",
        # Chat about an Episode
        "episode_chat"
    ];
    roles = [
        # Able to Chat about an Episode
        "reader",
        # Able to read chat Messages about an Episode
        "guest"
    ];

    "episode_read_chat" if "reader";

    "episode_chat" if "guest";
    "reader" if "guest";
}

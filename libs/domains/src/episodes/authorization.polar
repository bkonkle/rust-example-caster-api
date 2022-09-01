has_role(user: User, role_key: String, episode: Episode) if
  role in user.roles and
  role.role_key = role_key and
  role.resource_table = "episodes" and
  role.resource_id = episode.id;

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

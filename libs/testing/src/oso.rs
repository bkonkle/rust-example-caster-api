use mockall::mock;

mock! {
    pub Oso {
        /// High level interface for authorization decisions. Makes an allow query with the given
        /// actor, action and resource and returns true or false.
        pub fn is_allowed<
            Actor: 'static,
            Action: 'static,
            Resource: 'static,
        >(
            &self,
            actor: Actor,
            action: Action,
            resource: Resource,
        ) -> Result<bool, oso::OsoError>;
    }
}

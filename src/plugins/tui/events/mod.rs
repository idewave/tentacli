pub mod traits;

#[macro_export]
macro_rules! events_runtime {
    (
        impl EventsRuntime for $T:ty {
            $(local_events = [$($LE:ty),* $(,)?];)?
            $(parent_events = [$($GE:ty),* $(,)?];)?
            $(children = [$($child:ident),* $(,)?];)?
        }
    ) => {
        events_runtime!(@expand
            impl EventsRuntime for $T {
                local_events = [$($($LE),*)?];
                parent_events = [$($($GE),*)?];
                children = [$($($child),*)?];
            }
        );
    };

    (@expand
        impl EventsRuntime for $T:ty {
            local_events = [$($LE:ty),*];
            parent_events = [$($GE:ty),*];
            children = [$($child:ident),*];
        }
    ) => {
        impl EventsRuntime for $T {
            async fn register_all(
                &mut self,
                emitter: Arc<AsyncEventEmitter>,
            ) -> anyhow::Result<()> {

                // bind event system (parent emitter)
                self.bind_event_system(emitter.clone());

                // register LOCAL events
                $(
                    <Self as EventHandler<$LE>>::register_event(
                        self,
                        crate::plugins::tui::events::traits::EmitType::Local
                    ).await?;
                )*

                // register GLOBAL events
                $(
                    <Self as EventHandler<$GE>>::register_event(
                        self,
                        crate::plugins::tui::events::traits::EmitType::Parent
                    ).await?;
                )*

                // register children (each child gets THIS component's local emitter as parent)
                $(
                    let child_emitter = self.event_system().local_emitter.clone();
                    self.$child.register_as_child(child_emitter.clone()).await?;
                )*

                Ok(())
            }

            async fn try_update_all(&mut self) -> anyhow::Result<Vec<Echo>> {
                let mut output = Vec::new();

                // children first
                $(
                    output.extend(
                        self.$child.try_update_all().await?
                    );
                )*

                // then this component
                $(
                    output.extend(
                        <Self as EventHandler<$LE>>::try_update(
                            self,
                            crate::plugins::tui::events::traits::EmitType::Local
                        ).await?
                    );
                )*

                $(
                    output.extend(
                        <Self as EventHandler<$GE>>::try_update(
                            self,
                            crate::plugins::tui::events::traits::EmitType::Parent
                        ).await?
                    );
                )*

                Ok(output)
            }
        }
    };
}
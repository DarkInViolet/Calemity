import type { FormEvent } from "react";
import type { Conversation } from "../../lib/conversation";

interface ConversationSidebarProps {
    conversations: Conversation[];
    selectedConversationId: string | null;
    conversationTitle: string;
    onConversationTitleChange: (title: string) => void;
    onCreateConversation: () => Promise<void>;
    onSelectConversation: (conversationId: string) => void;
}

export function ConversationSidebar({
    conversations,
    selectedConversationId,
    conversationTitle,
    onConversationTitleChange,
    onCreateConversation,
    onSelectConversation,
}: ConversationSidebarProps) {
    function handleSubmit(event: FormEvent<HTMLFormElement>) {
        event.preventDefault();
        void onCreateConversation();
    }
    return (
        <aside className="conversation-sidebar">
            <div className="brand">
                <div className="brand-mark">C</div>

                <div>
                    <h1>Calemity</h1>
                    <span>Local messenger</span>
                </div>
            </div>

            <form
                className="new-conversation-form"
                onSubmit={handleSubmit}
            >
                <input
                    value={conversationTitle}
                    placeholder="New conversation"
                    aria-label="New conversation title"
                    maxLength={100}
                    onChange={(event) =>
                        onConversationTitleChange(event.target.value)
                    }
                />

                <button
                    type="submit"
                    aria-label="Create conversation"
                    disabled={!conversationTitle.trim()}
                >
                    +
                </button>
            </form>

            <div className="sidebar-section-heading">
                <span>Conversations</span>
                <span>{conversations.length}</span>
            </div>

            <nav
                className="conversation-list"
                aria-label="Conversations"
            >
                {conversations.length === 0 ? (
                    <div className="sidebar-empty-state">
                        <p>No conversations yet.</p>
                        <span>Create one above to begin.</span>
                    </div>
                ) : (
                    conversations.map((conversation) => {
                        const selected =
                            conversation.id === selectedConversationId;

                        return (
                            <button
                                key={conversation.id}
                                type="button"
                                className={`conversation-item ${
                                    selected ? "is-selected" : ""
                                }`}
                                aria-current={selected ? "page" : undefined}
                                onClick={() =>
                                    onSelectConversation(conversation.id)
                                }
                            >
                                <span className="conversation-avatar">
                                    {conversation.title
                                        .charAt(0)
                                        .toUpperCase()}
                                </span>

                                <span className="conversation-details">
                                    <strong>{conversation.title}</strong>
                                    <small>Local conversation</small>
                                </span>
                            </button>
                        );
                    })
                )}
            </nav>

            <div className="sidebar-footer">
                <span className="status-dot" />
                <span>Stored locally</span>
            </div>
        </aside>
    );
}
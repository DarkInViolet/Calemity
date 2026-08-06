import type { Conversation } from "../../lib/conversation";
interface ChatHeaderProps {
    conversation: Conversation | null;
}
export function ChatHeader({
    conversation,
}: ChatHeaderProps) {
    return (
        <header className="chat-header">
            <div>
                <span className="chat-header-eyebrow">
                    {conversation ? "Conversation" : "Welcome"}
                </span>

                <h2>
                    {conversation?.title ??
                        "Select or create a conversation"}
                </h2>
            </div>

            {conversation && (
                <div className="chat-header-status">
                    <span className="status-dot" />
                    Available offline
                </div>
            )}
        </header>
    );
}
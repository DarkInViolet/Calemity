import { useEffect, useRef } from "react";
import type { Conversation } from "../../lib/conversation";
import type { MessageView } from "../../lib/message";

interface MessageListProps {
    conversation: Conversation | null;
    messages: MessageView[];
}

function formatTimestamp(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleTimeString([], {
        hour: "2-digit",
        minute: "2-digit",
    });
}

export function MessageList({
    conversation,
    messages,
}: MessageListProps) {
    const endOfMessagesRef = useRef<HTMLDivElement | null>(null);

    useEffect(() => {
        endOfMessagesRef.current?.scrollIntoView({
            behavior: "smooth",
        });
    }, [messages]);

    if (!conversation) {
        return (
            <section className="message-list message-list-empty">
                <div className="empty-state-icon">C</div>
                <h3>Your conversations live here</h3>
                <p>
                    Create or select a conversation from the sidebar.
                </p>
            </section>
        );
    }

    if (messages.length === 0) {
        return (
            <section className="message-list message-list-empty">
                <div className="empty-state-icon">
                    {conversation.title.charAt(0).toUpperCase()}
                </div>
                <h3>{conversation.title}</h3>
                <p>This is the beginning of this conversation.</p>
            </section>
        );
    }

    return (
        <section
            className="message-list"
            aria-label={`Messages in ${conversation.title}`}
        >
            {messages.map((message) => (
                <article
                    key={message.id}
                    className="message"
                >
                    <div className="message-avatar">
                        {message.isOwn ? "Y" : "?"}
                    </div>

                    <div className="message-body">
                        <div className="message-metadata">
                            <strong>
                                {message.isOwn ? "You" : "Unknown"}
                            </strong>

                            <time>
                                {formatTimestamp(message.timestamp)}
                            </time>
                        </div>

                        <p>{message.content}</p>
                    </div>
                </article>
            ))}

            <div ref={endOfMessagesRef} />
        </section>
    );
}
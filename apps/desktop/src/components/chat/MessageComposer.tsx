import type { FormEvent } from "react";
interface MessageComposerProps {
    value: string;
    conversationTitle: string | null;
    disabled: boolean;
    onChange: (value: string) => void;
    onSend: () => Promise<void>;
}

export function MessageComposer({
    value,
    conversationTitle,
    disabled,
    onChange,
    onSend,
}: MessageComposerProps) {
    function handleSubmit(event: FormEvent<HTMLFormElement>) {
        event.preventDefault();
        void onSend();
    }

    return (
        <footer className="composer-container">
            <form
                className="message-composer"
                onSubmit={handleSubmit}
            >
                <input
                    value={value}
                    disabled={disabled}
                    aria-label="Message"
                    placeholder={
                        conversationTitle
                            ? `Message ${conversationTitle}`
                            : "Select a conversation first"
                    }
                    onChange={(event) =>
                        onChange(event.target.value)
                    }
                />

                <button
                    type="submit"
                    disabled={disabled || !value.trim()}
                >
                    Send
                </button>
            </form>
        </footer>
    );
}
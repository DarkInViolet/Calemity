import { useEffect, useState } from "react";

import { loadMessages, sendMessage } from "./lib/tauri";
import type { Message } from "./lib/message";

const AUTHOR =
"01K1GK79HVM0R8Y8B5XJXQ6A1A";

const CONVERSATION =
"01K1GK79HVM0R8Y8B5XJXQ6A1B";

export default function App() {

    const [messages, setMessages] =
    useState<Message[]>([]);

    const [text, setText] =
    useState("");

    async function refresh() {

        const data =
        await loadMessages(CONVERSATION);

        setMessages(data as Message[]);
    }

    async function handleSend() {

        if (!text.trim())
            return;

        await sendMessage(
            AUTHOR,
            CONVERSATION,
            "desktop",
            text
        );

        setText("");

        await refresh();
    }

    useEffect(() => {

        refresh();

    }, []);

    return (

        <div
        style={{
            padding: 30,
            fontFamily: "sans-serif",
            maxWidth: 600,
            margin: "auto",
        }}
        >

        <h1>Calemity</h1>

        <div
        style={{
            border: "1px solid gray",
            minHeight: 350,
            padding: 10,
            marginBottom: 15,
        }}
        >

        {messages.map(message => (

            <div key={message.id}>

            {message.content}

            </div>

        ))}

        </div>

        <input

        value={text}

        onChange={e =>

            setText(e.target.value)

        }

        style={{
            width: "80%",
        }}

        />

        <button
        onClick={handleSend}
        >

        Send

        </button>

        </div>

    );

}

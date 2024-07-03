import { info } from "tauri-plugin-log-api";

const GLAZEWM_IPC_ADDR = "ws://localhost:6123";

export const subscribe = (
	event: string,
	onMessage: (payload: unknown) => void,
) => {
	const ws = new WebSocket(GLAZEWM_IPC_ADDR);

	ws.onmessage = (event) => {
		const payload = JSON.parse(event.data);
		const isSubscription = payload?.messageType === "event_subscription";
		isSubscription && onMessage(payload);
	};

	ws.onopen = () => {
		ws.send(`subscribe -e ${event}`);
	};
};

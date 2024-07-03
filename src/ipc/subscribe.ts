import { info } from "tauri-plugin-log-api";

const GLAZEWM_IPC_ADDR = "ws://localhost:6123";

type Optional<T> = { [P in keyof T]?: Optional<T[P]> };

export type FocusChangedPayload = {
	data: {
		focusedContainer: {
			handle: number;
		};
	};
};

export type Event = "focus_changed";

export const subscribe = <T>(
	event: Event,
	onMessage: (payload: Optional<T>) => void,
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

	return ws;
};

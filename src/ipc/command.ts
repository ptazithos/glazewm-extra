import type { Optional } from "./utils";
import { GLAZEWM_IPC_ADDR } from "./utils";

type Command = "windows" | "workspaces";

export const command = <T>(command: Command): Promise<Optional<T>> => {
	return new Promise((resolve, reject) => {
		const ws = new WebSocket(GLAZEWM_IPC_ADDR);

		ws.onopen = () => {
			ws.send(command);
		};

		ws.onmessage = (event) => {
			resolve(JSON.parse(event.data) as Optional<T>);
			ws.close();
		};

		ws.onerror = () => {
			reject();
			ws.close();
		};
	});
};

type Payload<T> = {
	data: T;
};

type Window = {
	handle: number;
};
type WindowsPayload = Payload<Array<Window>>;

export const getWindows = async () =>
	(await command<WindowsPayload>("windows")).data ?? [];

type WindowSpace = {
	handle: number;
	sizePercentage: number;
};
type Workspace = {
	tilingDirection: "horizontal" | "vertical";
	sizePercentage: number;
	children: Array<Workspace | WindowSpace>;
};

type WorkspacesPayload = Payload<Array<Workspace>>;

export const getWorkspace = async () =>
	(await command<WorkspacesPayload>("workspaces")).data ?? [];

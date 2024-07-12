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

export type WindowSpace = {
	handle: number;
	type: "window";
	sizePercentage: number;
};

export type SplitSpace = {
	sizePercentage: number;
	type: "split";
	children: Array<SplitSpace | WindowSpace>;
	tilingDirection: "horizontal" | "vertical";
};

export type Workspace = {
	tilingDirection: "horizontal" | "vertical";
	sizePercentage: number;
	focusIndex: number;
	type: "workspace";
	children: Array<SplitSpace | WindowSpace>;
};

export type WorkspacesPayload = Payload<Array<Workspace>>;

export const getWorkspaces = async () =>
	(await command<WorkspacesPayload>("workspaces")).data ?? [];

import React from "react";
import ReactDOM from "react-dom/client";

const OverviewApp = () => {
	return <></>;
};

const rootEl = document.getElementById("root");
if (rootEl) {
	const root = ReactDOM.createRoot(rootEl);
	root.render(
		<React.StrictMode>
			<OverviewApp />
		</React.StrictMode>,
	);
}

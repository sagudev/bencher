import {
	type Accessor,
	type Resource,
	Show,
	createEffect,
	createSignal,
} from "solid-js";
import {
	type JsonAuthUser,
	type JsonPerfQuery,
	type JsonProject,
	Visibility,
} from "../../../../types/bencher";
import { setPageTitle } from "../../../../util/resource";
import ShareModal from "./ShareModal";
import PinModal from "./PinModal";

export interface Props {
	apiUrl: string;
	isConsole: boolean;
	user: JsonAuthUser;
	project: Resource<JsonProject>;
	isPlotInit: Accessor<boolean>;
	perfQuery: Accessor<JsonPerfQuery>;
	handleRefresh: () => void;
}

const PerfHeader = (props: Props) => {
	const [share, setShare] = createSignal(false);
	const [pin, setPin] = createSignal(false);

	createEffect(() => {
		setPageTitle(props.project()?.name);
	});

	return (
		<div class="columns">
			<div class="column">
				<h1 class="title is-3" style="word-break: break-word;">
					{props.project()?.name}
				</h1>
			</div>
			<ShareModal
				apiUrl={props.apiUrl}
				user={props.user}
				perfQuery={props.perfQuery}
				isPlotInit={props.isPlotInit}
				project={props.project}
				share={share}
				setShare={setShare}
			/>
			<PinModal
				apiUrl={props.apiUrl}
				user={props.user}
				perfQuery={props.perfQuery}
				isPlotInit={props.isPlotInit}
				project={props.project}
				share={pin}
				setShare={setPin}
			/>
			<div class="column is-narrow">
				<nav class="level">
					<div class="level-right">
						<Show when={props.project()?.url}>
							<div class="level-item">
								<a
									class="button is-fullwidth"
									title={`View ${props.project()?.name} website`}
									href={props.project()?.url ?? ""}
									rel="noreferrer nofollow"
									target="_blank"
								>
									<span class="icon">
										<i class="fas fa-globe" />
									</span>
									<span>Website</span>
								</a>
							</div>
						</Show>
						<Show when={!props.isPlotInit()}>
							<nav class="level is-mobile">
								<Show when={props.project()?.visibility === Visibility.Public}>
									<div class="level-item">
										<button
											class="button is-fullwidth"
											type="button"
											title={`Share ${props.project()?.name}`}
											onClick={(e) => {
												e.preventDefault();
												setShare(true);
											}}
										>
											<span class="icon">
												<i class="fas fa-share" />
											</span>
											<span>Share</span>
										</button>
									</div>
								</Show>

								<Show when={props.project()?.visibility === Visibility.Public}>
									<div class="level-item">
										<button
											class="button is-fullwidth"
											type="button"
											title={`Pin to ${props.project()?.name} dashboard`}
											onClick={(e) => {
												e.preventDefault();
												setPin(true);
											}}
										>
											<span class="icon">
												<i class="fas fa-thumbtack" />
											</span>
											<span>Pin</span>
										</button>
									</div>
								</Show>

								<div class="level-item">
									<button
										class="button is-fullwidth"
										type="button"
										title="Refresh Query"
										onClick={(e) => {
											e.preventDefault();
											props.handleRefresh();
										}}
									>
										<span class="icon">
											<i class="fas fa-sync-alt" />
										</span>
										<span>Refresh</span>
									</button>
								</div>
							</nav>
						</Show>
					</div>
				</nav>
			</div>
		</div>
	);
};

export default PerfHeader;

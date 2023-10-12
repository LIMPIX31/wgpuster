import type { FC }            from 'react'

import      { BrowserRouter } from 'react-router-dom'

export function withRouter(Component: FC) {
	return function () {
		return (
			<BrowserRouter>
				<Component />
			</BrowserRouter>
		)
	}
}

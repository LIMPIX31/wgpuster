import { Home as HomePage }    from '@page/home'
import { Route }               from 'react-router-dom'
import { Routes as DomRoutes } from 'react-router-dom'

export function Routes() {
	return (
		<DomRoutes>
			<Route index element={<HomePage />} />
		</DomRoutes>
	)
}

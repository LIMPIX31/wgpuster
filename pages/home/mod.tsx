import { useEffect } from 'react'
import { useId }     from 'react'

import init          from '@crate/main'
import { run }       from '@crate/main'

export function Home() {
	const id = useId()

	useEffect(() => {
		init().then(() => run(id))
	}, [id])

	return <div id={id} />
}

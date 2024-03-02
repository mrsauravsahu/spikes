import { useEffect, useState } from 'react'
import './App.css'

import { Bill as BillComponent, type BillProp } from './components/Bill'
import { getBills } from './apis/get-bills'
import { ErrorSnack } from './components/ErrorSnack'

export default function App() {
  const [bills, setBills] = useState<Array<BillProp>>([])
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    getBills({ limit: 10, offset: 0 }, (err: any, bills: Array<BillProp>) => {
      if (!err) setBills(bills)
      setError(err)
    })
  }, [])

  return (
    <>
      <div className='App'>
        {bills.length > 0 && <h1>your bills</h1>}
        {bills.map(bill => <BillComponent key={bill.id} bill={bill} />)}
        {error && <ErrorSnack message={error} />}
      </div>
    </>
  )
}

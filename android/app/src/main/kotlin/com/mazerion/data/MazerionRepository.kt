package com.mazerion.data

import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import uniffi.mazerion_ffi.*

class MazerionRepository {

    suspend fun getVersion(): String = withContext(Dispatchers.IO) {
        version()
    }

    suspend fun listCalculators(): List<CalculatorInfo> = withContext(Dispatchers.IO) {
        listCalculators()
    }

    suspend fun getCategories(): Map<String, Int> = withContext(Dispatchers.IO) {
        val categoryMap = getCategories()
        categoryMap.entries.associate { it.category to it.count.toInt() }
    }

    suspend fun getCalculatorsByCategory(category: String): List<CalculatorInfo> =
        withContext(Dispatchers.IO) {
            getCalculatorsByCategory(category)
        }

    suspend fun executeCalculator(
        calculatorId: String,
        params: List<CalcParam>
    ): CalcResult = withContext(Dispatchers.IO) {
        executeCalculator(calculatorId, params)
    }

    suspend fun executeBatch(
        requests: List<BatchCalculatorRequest>
    ): List<BatchCalculatorResult> = withContext(Dispatchers.IO) {
        executeBatch(requests)
    }
}
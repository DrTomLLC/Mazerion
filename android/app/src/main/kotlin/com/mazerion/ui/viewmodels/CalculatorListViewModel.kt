package com.mazerion.ui.viewmodels

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.mazerion.data.MazerionRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import mazerion.CalculatorInfo

data class CalculatorListUiState(
    val calculators: List<CalculatorInfo> = emptyList(),
    val isLoading: Boolean = false,
    val error: String? = null
)

class CalculatorListViewModel(
    private val repository: MazerionRepository = MazerionRepository()
) : ViewModel() {

    private val _uiState = MutableStateFlow(CalculatorListUiState())
    val uiState: StateFlow<CalculatorListUiState> = _uiState.asStateFlow()

    init {
        loadCalculators()
    }

    fun loadCalculators() {
        viewModelScope.launch {
            _uiState.value = _uiState.value.copy(isLoading = true, error = null)

            repository.listCalculators()
                .onSuccess { calculators ->
                    _uiState.value = _uiState.value.copy(
                        calculators = calculators,
                        isLoading = false
                    )
                }
                .onFailure { error ->
                    _uiState.value = _uiState.value.copy(
                        error = error.message ?: "Unknown error",
                        isLoading = false
                    )
                }
        }
    }
}
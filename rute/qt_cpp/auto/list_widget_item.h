#pragma once
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// This file is auto-generated by rute_gen. DO NOT EDIT
///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#include "../rute_base.h"
#include "../rute_manual.h"
#include <QListWidgetItem>


///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRListWidgetItem : public QListWidgetItem {
    Q_OBJECT
public:
    WRListWidgetItem() : QListWidgetItem() { }
    virtual ~WRListWidgetItem() {
        if (m_delete_callback) {
             m_delete_callback(m_private_data);
         }
    }
    
    RUDeleteCallback m_delete_callback = nullptr;
    void* m_private_data = nullptr;
};